use crate::{
    ast_enum::{
        AssignNode, AstNodeAccept, BinaryNode, ExprAcceptMut, ExprNode, ExprVisitorMut,
        GroupingNode, LiteralNode, UnaryNode,
    },
    enum_stmt::{StmtAcceptorMut, StmtNode, StmtVisitorMut, VarNode},
    environment::Environment,
    token::{Token, TokenType},
    RunTimeError,
};

// TODO Need to remove Copy
#[derive(Debug)]
pub struct Interpreter {
    envrionment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            envrionment: Environment::new(),
        }
    }

    // Maps all Value's onto Value::Bool
    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Bool(b) => *b,
            _ => true,
        }
    }

    fn is_equal(&self, left: &Value, right: &Value) -> bool {
        match (left, right) {
            (Value::Number(l), Value::Number(r)) => l == r,
            (Value::String(l), Value::String(r)) => l == r,
            (Value::Bool(l), Value::Bool(r)) => l == r,
            (Value::Nil, Value::Nil) => true,
            _ => false,
        }
    }

    pub fn interpret(&mut self, stmts: Vec<StmtNode>) -> Result<(), RunTimeError> {
        for stmt in stmts {
            self.execute(&stmt)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &StmtNode) -> Result<(), RunTimeError> {
        stmt.accept(self)
    }
}

impl StmtVisitorMut for Interpreter {
    fn visit_expr(&mut self, expr_node: &ExprNode) -> Result<(), RunTimeError> {
        let _ = expr_node.accept_mut(self);
        Ok(())
    }

    fn visit_print(&mut self, expr_node: &ExprNode) -> Result<(), RunTimeError> {
        let eval = expr_node.accept_mut(self).unwrap();
        // replace with stdout
        println!("{}", eval);
        Ok(())
    }

    fn visit_var_dec(&mut self, var_node: &VarNode) -> Result<(), RunTimeError> {
        let name = var_node.name.lexeme.clone();

        let value = var_node.value_expr.accept_mut(self)?;
        self.envrionment.put(name, value);
        Ok(())
    }

    // fn visit_var_assign(&mut self, var_node: &VarNode) -> Result<(), RunTimeError> {
    //    let value = var_node.value_expr.accept_mut(self)?;
    //    self.envrionment.update( &var_node.name, value)?;
    //    Ok(())
    // }
}

impl ExprVisitorMut for Interpreter {
    type Output = Result<Value, RunTimeError>;

    fn visit_assign(&mut self, node: &AssignNode) -> Self::Output {
        // Look up variable value and return it
        let v = node.value.accept_mut(self)?;
        self.envrionment.assign(&node.name, v.clone())?;
        Ok(v)
    }
    fn visit_variable(&mut self, value: &Token) -> Self::Output {
        // Look up variable value and return it
        self.envrionment.get(value)
    }

    fn visit_binary(&mut self, value: &BinaryNode) -> Self::Output {
        let left_eval = value.left.accept_mut(self)?;
        let right_eval = value.right.accept_mut(self)?;

        match value.operator.t_type {
            TokenType::Plus => match (left_eval, right_eval) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{l}{r}"))),
                _ => panic!("Addition not defined"),
            },
            TokenType::Minus => match (left_eval, right_eval) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
                _ => panic!("Addition not defined"),
            },
            TokenType::Star => match (left_eval, right_eval) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
                _ => panic!("Addition not defined"),
            },
            TokenType::Slash => match (left_eval, right_eval) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
                _ => panic!("Addition not defined"),
            },
            TokenType::Less => match (left_eval, right_eval) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l < r)),
                _ => panic!("Addition not defined"),
            },
            TokenType::LessEqual => match (left_eval, right_eval) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l <= r)),
                _ => panic!("Addition not defined"),
            },
            TokenType::Greater => match (left_eval, right_eval) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l > r)),
                _ => panic!("Addition not defined"),
            },
            TokenType::GreaterEqual => match (left_eval, right_eval) {
                (Value::Number(l), Value::Number(r)) => Ok(Value::Bool(l >= r)),
                _ => panic!("Addition not defined"),
            },
            TokenType::EqualEqual => Ok(Value::Bool(self.is_equal(&left_eval, &right_eval))),
            TokenType::BangEqual => Ok(Value::Bool(!self.is_equal(&left_eval, &right_eval))),
            _ => unimplemented!("Binary operatror not matched"),
        }
    }

    fn visit_unary(&mut self, value: &UnaryNode) -> Self::Output {
        let eval = value.right.accept_mut(self)?;
        match value.operator.t_type {
            TokenType::Bang => Ok(Value::Bool(self.is_truthy(&eval))),
            TokenType::Minus => match eval {
                Value::Number(n) => Ok(Value::Number(-n)),
                v => Err(RunTimeError::InvalidBangValue(
                    value.operator.clone(),
                    format!("Received {v:?}"),
                )),
            },
            _ => {
                panic!("Parser placed a none unary operator in a unary none {value:?}")
            }
        }
    }

    fn visit_grouping(&mut self, value: &GroupingNode) -> Self::Output {
        value.inner.accept_mut(self)
    }

    fn visit_literal(&mut self, value: &LiteralNode) -> Self::Output {
        match value {
            LiteralNode::String(s) => Ok(Value::String(s.clone())),
            LiteralNode::Number(n) => Ok(Value::Number(*n)),
            LiteralNode::True => Ok(Value::Bool(true)),
            LiteralNode::False => Ok(Value::Bool(false)),
            LiteralNode::Nil => Ok(Value::Nil),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Nil,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{n}"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "{s}"),
            Value::Nil => write!(f, "nil"),
        }
    }
}
