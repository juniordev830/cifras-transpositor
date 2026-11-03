use std::{env,io,process};
fn maze(size:usize,mut seed:u64)->Result<Vec<Vec<char>>,&'static str>{
    if !(5..=81).contains(&size) || size%2==0{return Err("Tamanho deve ser ímpar, entre 5 e 81");}
    let mut grid=vec![vec!['#';size];size];grid[1][1]=' ';let mut stack=vec![(1usize,1usize)];
    while let Some(&(x,y))=stack.last(){
        let choices:Vec<_>=[(-2i32,0i32),(2,0),(0,-2),(0,2)].iter().filter_map(|(dx,dy)|{
            let nx=x as i32+dx;let ny=y as i32+dy;
            if nx>0 && ny>0 && nx<(size-1) as i32 && ny<(size-1) as i32 && grid[ny as usize][nx as usize]=='#'{Some((nx as usize,ny as usize))}else{None}
        }).collect();
        if choices.is_empty(){stack.pop();continue;}
        seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let (nx,ny)=choices[(seed%choices.len() as u64) as usize];
        grid[(y+ny)/2][(x+nx)/2]=' ';grid[ny][nx]=' ';stack.push((nx,ny));
    }
    Ok(grid)
}
fn draw(grid:&[Vec<char>],player:(usize,usize)){
    for (y,row) in grid.iter().enumerate(){println!("{}",row.iter().enumerate().map(|(x,c)|if (x,y)==player{'@'}else if (x,y)==(grid.len()-2,grid.len()-2){'E'}else{*c}).collect::<String>());}
}
fn run()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<String>=env::args().skip(1).collect();
    if args.len()<2 || args.len()>3 || (args.len()==3 && args[2]!="--jogar"){return Err("Uso: labirinto-procedural TAMANHO SEMENTE [--jogar]".into());}
    let size=args[0].parse()?;let grid=maze(size,args[1].parse()?)?;let mut player=(1usize,1usize);draw(&grid,player);
    if args.len()==3{println!("WASD + Enter move; Q + Enter sai. E é a saída.");loop{
        let mut command=String::new();if io::stdin().read_line(&mut command)?==0{break;}
        let delta=match command.trim().to_lowercase().as_str(){"w"=>(0,-1),"s"=>(0,1),"a"=>(-1,0),"d"=>(1,0),"q"=>break,_=>continue};
        let (x,y)=((player.0 as i32+delta.0) as usize,(player.1 as i32+delta.1) as usize);
        if grid.get(y).and_then(|row|row.get(x)).map(|c|*c!='#').unwrap_or(false){player=(x,y);}
        draw(&grid,player);if player==(size-2,size-2){println!("Você venceu!");break;}
    }}Ok(())
}
fn main(){if let Err(e)=run(){eprintln!("Erro: {e}");process::exit(2);}}
#[cfg(test)]mod tests{use super::*;use std::collections::HashSet;
    #[test]fn reproducible_and_connected(){for seed in 0..20{
        let grid=maze(21,seed).unwrap();assert_eq!(grid,maze(21,seed).unwrap());let mut seen=HashSet::new();let mut stack=vec![(1usize,1usize)];
        while let Some((x,y))=stack.pop(){if !seen.insert((x,y)){continue;}for (dx,dy) in [(1,0),(-1,0),(0,1),(0,-1)]{let nx=(x as i32+dx) as usize;let ny=(y as i32+dy) as usize;if grid.get(ny).and_then(|row|row.get(nx))==Some(&' ') && !seen.contains(&(nx,ny)){stack.push((nx,ny));}}}
        assert!(seen.contains(&(19,19)));assert_eq!(seen.len(),grid.iter().flatten().filter(|c|**c==' ').count());}}
    #[test]fn reject_bad_dimensions(){assert!(maze(4,1).is_err());assert!(maze(83,1).is_err());}
}
