use aatree::AATreeMap;
use eq_float::F64;
pub fn getcollisions(power : i32) -> AATreeMap<F64,((f64,f64),(f64,f64))> 
{
    // Define starting positions, velocities, and masses
    let mut p1 = 0.02;
    let mut p2 = 0.05;
    let mut v1 = 0.0;
    let mut v2 = -1.0;
    let m1 = 1.0;
    let m2 = 100.0_f64.powi(power);
    let mut count : u64 = 0;
    let mut elapsedtime : F64 = F64::from(0.0);
    let wp = 0.0;
    let mut map : AATreeMap<F64,((f64,f64),(f64,f64))> = AATreeMap::new();
    map.insert(elapsedtime,((p1,v1),(p2+0.1,v2)));
    loop
    {
        // Calculate the time to next collision with the wall or other block
        let wallcolltime : f64 = (wp-p1)/v1;
        let blockcolltime : f64 = (p2-p1)/(v1-v2);
        // Negative time values are invalid as it means they are separating and will not collide next
        let wvalid = wallcolltime.is_finite() && wallcolltime > 0.0;
        let bcvalid = blockcolltime.is_finite() && blockcolltime > 0.0;
        let nextcol : Collision;
        // Find the earliest valid collision
        if wvalid && bcvalid
        {
            if blockcolltime < wallcolltime
            {
                nextcol = Collision::Blocks;
            }
            else {
                nextcol = Collision::Wall;
            }
        }
        else if wvalid
        {
            nextcol = Collision::Wall;
        }
        else if bcvalid{
            nextcol = Collision::Blocks;
        }
        else {
            break;
        }
        // Apply the time step and change the velocities accordingly
        match nextcol
        {
            Collision::Wall => 
            {
                let timestep = wallcolltime;
                elapsedtime.0 += timestep;
                p1 = 0.0;
                p2 += v2 * timestep;
                v1 = -v1;
            }
            Collision::Blocks=> 
            {
                let timestep = blockcolltime;
                elapsedtime.0 += timestep;
                p1 += v1 * timestep;
                p2 = p1;
                let newv1 = (m1-m2)/(m1+m2)*v1+(2.0*m2)/(m1+m2)*v2;
                v2 = (2.0*m1)/(m1+m2)*v1+(m2-m1)/(m1+m2)*v2;
                v1 = newv1;
            }
        }
        // Insert the positions and velocities into the lookup tree.
        map.insert(elapsedtime,((p1,v1),(p2+0.1,v2)));
        count += 1;
    }
    println!("There were {} total collisions",count);
    map
}
enum Collision
{
    Wall,
    Blocks
}