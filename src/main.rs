use glowing_happiness::*;

fn main() {
    let root = ".";
    let files = walk(root);
    let path_by_tool = collect_by_path(files);
    println!("{:?}", &path_by_tool);

    let counted_by_tool = count_by_path(&path_by_tool);
    println!("{:?}", &counted_by_tool);
    let tools = Vec::from_iter(counted_by_tool.into_keys());
    println!("{:?}", &tools);
}
