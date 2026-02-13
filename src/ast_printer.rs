use crate::expr::{Visitor, Expr, Binary, Grouping, Unary, Variable, walk_expr};
use crate::token::{Literal};


pub struct AstPrinter;
impl Visitor<String> for AstPrinter{
    fn visit_binaryexp(&self, e: &Binary) -> String {
        self.paranthesize(&e.op.lexeme, vec![&e.left, &e.right])

    }

    fn visit_groupingexp(&self, e: &Grouping) -> String {
        self.paranthesize("group", vec![&e.expression])

    }

    fn visit_literalexp(&self, e: &Literal) -> String {
        match e {
            Literal::Nil => "nil".to_string(),
            _ => e.to_string()
        }
    }

    fn visit_unaryexp(&self, e: &Unary) -> String {
        self.paranthesize(&e.op.lexeme, vec![&e.right])

    }

    fn visit_variableexp(&self, e: &Variable) -> String {
        e.name.lexeme.clone()
    }
}
impl AstPrinter {
    pub fn print(&self, expr: &Expr) -> String{
        return walk_expr(self, expr)
    }

    fn paranthesize(&self, name: &str, exprs: Vec<&Expr>) -> String{
        let mut s = String::new();

        s.push_str("(");
        s.push_str(name);

        for expr in exprs {
            s.push_str(" ");
            s.push_str(&walk_expr(self, expr));
        }

        s.push_str(")");
        s
    }
}
