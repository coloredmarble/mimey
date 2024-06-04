use std::collections::HashSet;

fn main() {
    /*
    // fuck performance
    let mut x: HashMap<&str, Vec<String>> = collections::HashMap::new();
    s.split('\n').for_each(|l| {
        let v = l.split_ascii_whitespace().collect::<Vec<_>>();
        let vn = v[0].strip_prefix('.').unwrap();
        if x.contains_key(vn) {
            x.get_mut(vn).unwrap().push(v[1..].join(";"));
            return;
        }
        x.insert(vn, [v[1..].join(";")].to_vec());
    });
    let mut nv = x.into_iter().collect::<Vec<(&str, Vec<String>)>>();
    nv.sort_by(|x, z| x.0.cmp(z.0));
    nv.into_iter().for_each(|(x,z)| {
        println!(",(\"{x}\", &{:?})",z);
    });
    */
    let mut h: HashSet<&str> = HashSet::new();
    mimey::EXTENSION_MIME.iter().for_each(|(n, _)| {
        if h.contains(n) {
            println!("{n}");
        }

        h.insert(n);
    });
    mimey::MYRIAD_EXTENSION_MIME.iter().for_each(|(x, _)| {
        if !h.contains(x) {
            println!("{x}");
        }
    });
}
