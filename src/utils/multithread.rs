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

    // We also need to know the dimensions of the view plane
    pub fn render_to_buffer(&mut self,
                        world: Arc<World>,
                        camera: &dyn Camera,
                        buffer: &mut RenderBuffer)
    {
        let vp_hres = 800_usize;
        let vp_wres = 600_usize;

        // let queue = ComputeQueue::new(Vector2::new(vp_wres, vp_hres),BLOCK_DIM);
        // let mut buffer = RenderBuffer::new((vp_hres, vp_wres), (100, 100));
        let n_thread = self.m_pool.thread_count();

        self.m_pool.scoped(|scoped|
            {
                for blockmeta in buffer.iter()
                {
                    scoped.execute(| |
                        {
                            buffer.read(camera.render(world.clone(), &blockmeta.clone()), &blockmeta);
                        });
                }
                /*
                for _ in 0..n_thread
                {
                    let q = &queue;
                    scope.execute(move ||
                        {

                        });
                }

                 */
            });
    }
}

fn work(world: Arc<World>,
        camera: &dyn Camera,
        rendermeta: &RenderMeta)
{
    camera.render(world, rendermeta);
}