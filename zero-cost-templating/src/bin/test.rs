pub struct Template<Type, Partial, After> {
    pub t: Type,
    pub partial: Partial,
    pub after: After,
}

pub struct PartialBlockPartialTemplate0;
pub struct PartialBlockPartialTemplate2;
// we should try this generically because then templates could be used cross-crate
impl<PartialType, PartialPartial, PartialAfter, After>
    Template<
        PartialBlockPartialTemplate0,
        Template<PartialType, PartialPartial, PartialAfter>,
        After,
    >
{
    pub fn partial_block_partial_template0(
        self,
    ) -> Template<PartialType, (), Template<PartialBlockPartialTemplate2, (), After>> {
        todo!()
    }
}

pub fn main() {}
