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
use crate::sampler::Sampler;

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

    // We also need to know the dimensions of the view plane
    fn render_scene(&mut self,
                    world: Arc<World>,
                    img_writer: &mut dyn OutputManager,
                    camera: &mut dyn Camera)
    {
        let vp_hres = 800;
        let vp_wres = 600;

        let queue = ComputeQueue::new(Vector2::new(vp_wres, vp_hres),
                                                    BLOCK_DIM);
        let n_thread = self.m_pool.thread_count();

        self.m_pool.scoped(|scope|
            {
                for _ in 0..n_thread
                {
                    let q = &queue;
                    scope.execute(move ||
                        {

                        });
                }
            });
    }
}

fn work(queue: &ComputeQueue,
        world: Arc<World>,
        camera: &Camera,
        renderbuffer: &RenderBuffer,
        sampler: &mut Sampler)
{
    let block_dim = queue.get_block_dim();
    for block in queue.iter()
    {

    }
}