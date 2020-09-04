let tracer = Whitted::new();

let mut boxed_vp = Box::new(ViewPlane::new());
let vp_hres = 400;
let vp_vres = 400;
boxed_vp.m_hres = vp_hres;
boxed_vp.m_vres = vp_vres;
boxed_vp.m_pixsize = 0.5;
boxed_vp.m_numsample = 16;

let mut imgwriter = ImageWriter::new("test.jpg", vp_hres, vp_vres);
let mut world = World::new(boxed_vp);


let mut sphereA = Arc::new(Mutex::new(Sphere::new(10.0,
Vector3::new(12.0, 20.0, 15.0),
Colorf::new(0.0, 1.0, 0.0))));
let mut sphereB = Arc::new(Mutex::new(Sphere::new(15.0,
Vector3::new(30.0, 10.0, 20.0),
Colorf::new(1.0, 0.0, 0.0))));
let mut sphereC = Arc::new(Mutex::new(Sphere::new(5.0,
Vector3::new(30.0, 10.0, 10.0),
Colorf::new(0.0, 1.0, 1.0))));
world.add_object(sphereA);
world.add_object(sphereB);
world.add_object(sphereC);

let c = vec![COLOR_BLUE, COLOR_RED, Colorf::new(0.0, 1.0, 1.0)];
let objlen= world.m_objects.len();
let materials: Vec<Phong> = (0..objlen)
.collect::<Vec<_>>()
.iter()
.map(|x| setUpMaterial(1.0/(*x) as f32, 0.3 * (*x) as f32, 0.5))
.collect::<Vec<Phong>>();

for i in 0..objlen
{
let mut obj = world.m_objects[i].lock().unwrap();
obj.set_material(Arc::new(materials[i].clone()));
}

setUpLights(&mut world);
let mut ph = setUpCamera();
ph.m_distance_from_vp = 50.0;
ph.m_zoom = 1.0;
ph.m_core.m_exposure_time = 0.05;
let worldptr = Arc::new(world);
ph.render_scene(worldptr, &tracer, &mut imgwriter,1.0);
imgwriter.output();