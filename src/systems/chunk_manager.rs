use log::info;
use nalgebra_glm as glm;

use crate::{chunk::Chunk, utils::world_to_chunk_position};

use super::chunk_builder::ChunkGenStrategy;

pub const CHUNK_LOAD_DISTANCE: i32 = 4;
pub const CHUNKS_TO_BUILT_PER_TICK: usize = 1;

/// If the chunk is currently loaded or not
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    Loaded,
    Unloaded,
}

/// Manages all chunks near the player.
/// Automatically loads and unloads chunks as the player moves.
pub struct ChunkManager {
    /// All chunks that have been built, regardless of whether they are loaded or not.
    pub chunks: Vec<(Chunk, ChunkState)>,

    /// The current chunk that the player is in.
    pub current_chunk: (i32, i32),

    /// The strategy to use for generating chunks.
    pub gen_strategy: ChunkGenStrategy,

    /// The queue of chunks that still need to be built.
    pub chunk_queue: Vec<(i32, i32)>,
}

impl ChunkManager {
    /// Creates a new chunk manager.
    pub fn new(gen_strategy: ChunkGenStrategy, player_pos: glm::Vec3) -> Self {
        let player_x = player_pos.x;
        let player_z = player_pos.z;

        let chunk_pos = world_to_chunk_position(player_x as i32, player_z as i32);

        Self {
            chunks: Vec::new(),
            current_chunk: chunk_pos,
            gen_strategy,
            chunk_queue: Vec::new(),
        }
    }

    /// Adds all of the chunks that need to be loaded to the queue.
    /// If a chunk is already loaded, it will not be added to the queue.
    fn add_chunks_to_queue(&mut self) {
        let chunks_to_load = self.get_chunks_around(self.current_chunk);
        let chunks_to_load = chunks_to_load
            .iter()
            .filter(|(cx, cz)| {
                !self
                    .chunks
                    .iter()
                    .any(|(chunk, _)| chunk.position == (*cx, *cz))
            })
            .filter(|(cx, cz)| !self.chunk_queue.contains(&(*cx, *cz)))
            .collect::<Vec<_>>();

        for (cx, cz) in chunks_to_load {
            self.chunk_queue.push((*cx, *cz));
        }
    }

    /// Unloads all chunks that are too far away from the player.
    fn unload_distant_chunks(&mut self) {
        self.chunks.iter_mut().for_each(|(chunk, state)| {
            let (cx, cz) = chunk.position;
            let (px, pz) = self.current_chunk;

            let dx = (cx - px).abs();
            let dz = (cz - pz).abs();

            if dx > CHUNK_LOAD_DISTANCE || dz > CHUNK_LOAD_DISTANCE {
                *state = ChunkState::Unloaded;
            }
        });
    }

    /// Loads the first n chunks per tick. This is to prevent lag spikes.
    fn build_next_chunks(&mut self) {
        for _ in 0..CHUNKS_TO_BUILT_PER_TICK {
            if let Some((cx, cz)) = self.chunk_queue.pop() {
                // Check if the chunk is already loaded
                if let Some((_, status)) = self
                    .chunks
                    .iter_mut()
                    .find(|(chunk, _)| chunk.position == (cx, cz))
                {
                    *status = ChunkState::Loaded;
                    continue;
                }

                let mut chunk = Chunk::new((cx, cz));
                self.gen_strategy.apply(&mut chunk);
                self.chunks.push((chunk, ChunkState::Loaded));
            }
        }
    }

    /// Gets a list of chunks around a certain chunk.
    fn get_chunks_around(&self, chunk_pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut chunks = Vec::new();
        let (cx, cz) = chunk_pos;

        for x in -CHUNK_LOAD_DISTANCE..=CHUNK_LOAD_DISTANCE {
            for z in -CHUNK_LOAD_DISTANCE..=CHUNK_LOAD_DISTANCE {
                chunks.push((cx + x, cz + z));
            }
        }

        assert!(chunks.len() == (CHUNK_LOAD_DISTANCE as usize * 2 + 1).pow(2));

        chunks
    }

    /// Adds all chunks that need to be loaded to the queue.
    pub fn update(&mut self, player_pos: glm::Vec3) {
        // Build chunks in the queue
        self.build_next_chunks();

        let x = player_pos.x as i32;
        let z = player_pos.z as i32;

        let current_chunk = world_to_chunk_position(x, z);

        if current_chunk != self.current_chunk {
            info!("Rebuilding chunks around {:?}", current_chunk);

            self.current_chunk = current_chunk;

            self.add_chunks_to_queue();
            self.unload_distant_chunks();
        }
    }
}
