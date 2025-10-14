use crate::{
    ast::type_annotation::TypeAnnotation,
    parse::{Parser, ParsingError},
};

impl<'a> Parser<'a> {
    pub fn parse_union_type_annotation(
        &mut self,
    ) -> Result<TypeAnnotation, ParsingError> {
        todo!()
    }
}
