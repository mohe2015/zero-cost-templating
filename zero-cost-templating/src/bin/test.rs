pub struct Template<Type, Partial, End> {
    t: Type,
    partial: Partial,
    end: End,
}

pub struct PartialBlockPartialTemplate0;
pub struct PartialBlockPartialTemplate2;
// we should try this generically because then templates could be used cross-crate
impl<PartialType, PartialPartial, PartialEnd, End>
    Template<PartialBlockPartialTemplate0, Template<PartialType, PartialPartial, PartialEnd>, End>
{
    pub fn partial_block_partial_template0(
        template: Self,
    ) -> Template<PartialType, (), Template<PartialBlockPartialTemplate2, (), End>> {
        todo!()
    }
}

pub fn main() {}
