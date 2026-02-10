use crate::parse::*;

pub fn flatten(v: Vec<(AST, AST)>) -> (Vec<Equation>, Vec<String>) {
    let mut funmap = Vec::new();

    let mut eqs = Vec::new();
    for (l, r) in v {
        let mut varmap = Vec::new();
        let lhs = flatten_ast(l, &mut funmap, &mut varmap);
        let rhs = flatten_ast(r, &mut funmap, &mut varmap);
        eqs.push(Equation { lhs, rhs, oriented: false });
    }
    (eqs, funmap)
}

fn flatten_ast(ast: AST, funmap: &mut Vec<String>, varmap: &mut Vec<String>) -> Box<FlatTerm> {
    let mut flat = Vec::new();
    flatten_ast_impl(&ast, funmap, varmap, &mut flat);
    flat.into_boxed_slice()
}

fn flatten_ast_impl(ast: &AST, funmap: &mut Vec<String>, varmap: &mut Vec<String>, flat: &mut Vec<Entry>) {
    let f = ast.f.to_string();
    let is_var = ('A'..='Z').contains(&f.chars().next().unwrap());
    let sym = if is_var {
        assert!(ast.children.is_empty());

        if let Some(n) = funmap.iter().position(|x| *x == f) {
            let n = n as i32;
            Sym { repr: -(n+1) }
        } else {
            let n = varmap.len() as i32;
            let sym = Sym { repr: -(n+1) };
            varmap.push(f);
            sym
        }
    } else {
        if let Some(n) = funmap.iter().position(|x| *x == f) {
            Sym { repr: n as i32 }
        } else {
            let n = funmap.len() as i32;
            let sym = Sym { repr: n };
            funmap.push(f);
            sym
        }
    };

    flat.push(Entry {
        sym,
        size: size(ast),
    });
    for c in &ast.children {
        flatten_ast_impl(c, funmap, varmap, flat);
    }
}

fn size(x: &AST) -> u32 {
    x.children.iter().map(size).sum::<u32>() + 1
}
