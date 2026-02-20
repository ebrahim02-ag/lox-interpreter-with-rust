use crate::expr::{Visitor,Assign,  Expr, Binary, Grouping, Unary, Variable, walk_expr};
use crate::stmt::{Stmt, Visitor as StmtVisitor, Expression, Print, walk_stmt, Variable as StmVariable, Block, If};
use crate::token::{Literal};
pub struct AstPrinter;


impl StmtVisitor<String> for AstPrinter {
    fn visit_block_stmt(&self, e: &Block) -> String {
        let mut s = String::new();
        for i in self.print_stmts(&e.statements){
            s = s + i.as_str();
        }
        s
    }

    fn visit_expression(&self, e: &Expression) -> String {
        self.print(&e.expression)
    }

    fn visit_print(&self, e: &Print) -> String {
        format!("print {}",self.print(&e.expression))
    }

    fn visit_var_stmt(&self, e: &StmVariable) -> String {
        format!("var {} = {}", e.name.lexeme, self.print(&e.initializer))
    }

    fn visit_if_stmt(&self, e: &If) -> String {
        let mut s = format!("if ({}) {}", self.print(&e.condition), walk_stmt(self, &e.then_branch));
        // if else exists then mutate s and add the esle to it
        e.else_branch.as_ref().map(|e| s = format!("{} else {}", s, walk_stmt(self, &e)));
        s
    }
}

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
        format!("var {} =", e.name.lexeme)
    }

    fn visit_assignexp(&self, e: &Assign) -> String {
        format!("var {} =", e.name.lexeme)
    }
}


impl AstPrinter {
    pub fn print_stmts(&self, stmts: &Vec<Stmt>) -> Vec<String>{
        let mut exprs = Vec::new();
        for statement in stmts{
            exprs.push(walk_stmt(self, &statement))
        }
        exprs
    }


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
