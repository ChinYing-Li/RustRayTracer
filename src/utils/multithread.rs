use std::iter;
use std::time::SystemTime;

use scoped_threadpool::Pool;
use crate::world::world::World;
use std::sync::Arc;
use crate::output::OutputManager;
use crate::render::cam::Camera;
use crate::utils::computequeue::ComputeQueue;
use cgmath::{Vector2, Vector3};
use crate::render::renderbuffer::RenderBuffer;
use crate::render::renderdata::RenderMeta;

const BLOCK_DIM: Vector2<u32> = Vector2::new(8, 8);

pub struct MultiThread
{
    m_pool: Pool
}

impl MultiThread
{
    pub fn new(n_threads: u32) -> MultiThread
    {
        MultiThread
        {
            m_pool: Pool::new(n_threads)
        }
    }

    pub fn get_thread_count(&self) -> u32
    {
        self.m_pool.thread_count()
    }

    // We also need to know the dimensions of the view plane
    pub fn render_to_buffer<'a>(&mut self,
                                world: Arc<World>,
                                camera: &dyn Camera,
                                buffer: &'a RenderBuffer)
    {
        let n_threads = self.get_thread_count();
        self.m_pool.scoped(|scoped|
            {
                for _ in 0..n_threads
                {
                    let b = &buffer;
                    let w = world.clone();
                    scoped.execute( move ||
                        {
                            work(w, camera, b);
                        }
                    );
                }
            });
    }
}

fn work<'a>(world: Arc<World>,
            camera: &dyn Camera,
            buffer: &'a RenderBuffer)
{
    for rendermeta in buffer.iter()
    {
        let samples = camera.render(world.clone(), &rendermeta);
        buffer.read(samples, &rendermeta);
    }
}