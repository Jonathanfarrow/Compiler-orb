use std::fs;
use std::io::Write;

fn main() {
    let output_dir = "src/ast";
    fs::create_dir_all(output_dir).unwrap();

    let ast_definitions = vec![
        ("BinaryExpr", vec!["Expr left", "Token operator", "Expr right"]),
        ("UnaryExpr", vec!["Token operator", "Expr right"]),
        ("LiteralValue", vec!["f64 number", "String string"]),
    ];

    let mut file = fs::File::create(format!("{}/expr.rs", output_dir)).unwrap();
    writeln!(file, "// Automatically generated AST").unwrap();
    writeln!(file, "pub enum Expr {{").unwrap();
    for (name, _) in &ast_definitions {
        writeln!(file, "    {}(Box<{}>),", name, name).unwrap();
    }
    writeln!(file, "}}").unwrap();

    for (name, fields) in &ast_definitions {
        writeln!(file, "pub struct {} {{", name).unwrap();
        for field in fields {
            let mut parts = field.splitn(2, ' ');
            let ty = parts.next().unwrap();
            let name = parts.next().unwrap();
            writeln!(file, "    pub {}: {},", name, ty).unwrap();
        }
        writeln!(file, "}}").unwrap();
    }
}
