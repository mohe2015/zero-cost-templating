#![feature(print_internals)]
#![feature(unsafe_pin_internals)]
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
extern crate alloc;
use futures_util::StreamExt;
use std::{borrow::Cow, io::Write, pin::pin};
use zero_cost_templating::{template_stream, TheStream};
pub async fn inner<After>(template: Tp<Tp60, (), After>) -> After {
    template.test("inner").await
}
#[must_use]
pub struct Tp<Type, Partial, After> {
    r#type: Type,
    partial: Partial,
    after: After,
}
#[automatically_derived]
impl<Type: ::core::clone::Clone, Partial: ::core::clone::Clone, After: ::core::clone::Clone>
    ::core::clone::Clone for Tp<Type, Partial, After>
{
    #[inline]
    fn clone(&self) -> Tp<Type, Partial, After> {
        Tp {
            r#type: ::core::clone::Clone::clone(&self.r#type),
            partial: ::core::clone::Clone::clone(&self.partial),
            after: ::core::clone::Clone::clone(&self.after),
        }
    }
}
#[automatically_derived]
impl<Type: ::core::marker::Copy, Partial: ::core::marker::Copy, After: ::core::marker::Copy>
    ::core::marker::Copy for Tp<Type, Partial, After>
{
}
#[must_use]
///AEmpty
pub struct Tp0;
#[automatically_derived]
impl ::core::clone::Clone for Tp0 {
    #[inline]
    fn clone(&self) -> Tp0 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp0 {}
#[must_use]
///BText
pub struct Tp1;
#[automatically_derived]
impl ::core::clone::Clone for Tp1 {
    #[inline]
    fn clone(&self) -> Tp1 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp1 {}
#[must_use]
///CElementWithAttribute
pub struct Tp2;
#[automatically_derived]
impl ::core::clone::Clone for Tp2 {
    #[inline]
    fn clone(&self) -> Tp2 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp2 {}
#[must_use]
///CElementWithContent
pub struct Tp3;
#[automatically_derived]
impl ::core::clone::Clone for Tp3 {
    #[inline]
    fn clone(&self) -> Tp3 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp3 {}
#[must_use]
///CEmptyElement
pub struct Tp4;
#[automatically_derived]
impl ::core::clone::Clone for Tp4 {
    #[inline]
    fn clone(&self) -> Tp4 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp4 {}
#[must_use]
///CSelfClosing
pub struct Tp5;
#[automatically_derived]
impl ::core::clone::Clone for Tp5 {
    #[inline]
    fn clone(&self) -> Tp5 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp5 {}
#[must_use]
///CSelfClosingBooleanAttr
pub struct Tp6;
#[automatically_derived]
impl ::core::clone::Clone for Tp6 {
    #[inline]
    fn clone(&self) -> Tp6 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp6 {}
#[must_use]
///DElementWithAttributeAndVariables
pub struct Tp7;
#[automatically_derived]
impl ::core::clone::Clone for Tp7 {
    #[inline]
    fn clone(&self) -> Tp7 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp7 {}
#[must_use]
///DVariable
pub struct Tp8;
#[automatically_derived]
impl ::core::clone::Clone for Tp8 {
    #[inline]
    fn clone(&self) -> Tp8 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp8 {}
#[must_use]
///EIfElse
pub struct Tp9;
#[automatically_derived]
impl ::core::clone::Clone for Tp9 {
    #[inline]
    fn clone(&self) -> Tp9 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp9 {}
#[must_use]
///EIfElseEmpty
pub struct Tp10;
#[automatically_derived]
impl ::core::clone::Clone for Tp10 {
    #[inline]
    fn clone(&self) -> Tp10 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp10 {}
#[must_use]
///EIfElseEmptyFalse
pub struct Tp11;
#[automatically_derived]
impl ::core::clone::Clone for Tp11 {
    #[inline]
    fn clone(&self) -> Tp11 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp11 {}
#[must_use]
///EIfElseEmptyTrue
pub struct Tp12;
#[automatically_derived]
impl ::core::clone::Clone for Tp12 {
    #[inline]
    fn clone(&self) -> Tp12 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp12 {}
#[must_use]
///EIfElseWithVariables
pub struct Tp13;
#[automatically_derived]
impl ::core::clone::Clone for Tp13 {
    #[inline]
    fn clone(&self) -> Tp13 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp13 {}
#[must_use]
///FEachEmpty
pub struct Tp14;
#[automatically_derived]
impl ::core::clone::Clone for Tp14 {
    #[inline]
    fn clone(&self) -> Tp14 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp14 {}
#[must_use]
///FEachOneVariable
pub struct Tp15;
#[automatically_derived]
impl ::core::clone::Clone for Tp15 {
    #[inline]
    fn clone(&self) -> Tp15 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp15 {}
#[must_use]
///FEachTwoVariables
pub struct Tp16;
#[automatically_derived]
impl ::core::clone::Clone for Tp16 {
    #[inline]
    fn clone(&self) -> Tp16 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp16 {}
#[must_use]
///FEachTwoVariablesHtml
pub struct Tp17;
#[automatically_derived]
impl ::core::clone::Clone for Tp17 {
    #[inline]
    fn clone(&self) -> Tp17 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp17 {}
#[must_use]
///GEmptyTemplate
pub struct Tp18;
#[automatically_derived]
impl ::core::clone::Clone for Tp18 {
    #[inline]
    fn clone(&self) -> Tp18 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp18 {}
#[must_use]
///GEmptyTemplateMultiple
pub struct Tp19;
#[automatically_derived]
impl ::core::clone::Clone for Tp19 {
    #[inline]
    fn clone(&self) -> Tp19 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp19 {}
#[must_use]
///GOnlyPartialBlock
pub struct Tp20;
#[automatically_derived]
impl ::core::clone::Clone for Tp20 {
    #[inline]
    fn clone(&self) -> Tp20 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp20 {}
#[must_use]
///GPartialBlock
pub struct Tp21;
#[automatically_derived]
impl ::core::clone::Clone for Tp21 {
    #[inline]
    fn clone(&self) -> Tp21 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp21 {}
#[must_use]
///GPartialBlockPartial
pub struct Tp22;
#[automatically_derived]
impl ::core::clone::Clone for Tp22 {
    #[inline]
    fn clone(&self) -> Tp22 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp22 {}
#[must_use]
///GTemplateOnlyPartialBlock
pub struct Tp23;
#[automatically_derived]
impl ::core::clone::Clone for Tp23 {
    #[inline]
    fn clone(&self) -> Tp23 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp23 {}
#[must_use]
///GTemplateOnlyPartialBlockMultiple
pub struct Tp24;
#[automatically_derived]
impl ::core::clone::Clone for Tp24 {
    #[inline]
    fn clone(&self) -> Tp24 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp24 {}
#[must_use]
///GTemplateOnlyPartialBlockWithText
pub struct Tp25;
#[automatically_derived]
impl ::core::clone::Clone for Tp25 {
    #[inline]
    fn clone(&self) -> Tp25 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp25 {}
#[must_use]
///GTemplateOnlyPartialBlockWithTextMultiple
pub struct Tp26;
#[automatically_derived]
impl ::core::clone::Clone for Tp26 {
    #[inline]
    fn clone(&self) -> Tp26 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp26 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariable
pub struct Tp27;
#[automatically_derived]
impl ::core::clone::Clone for Tp27 {
    #[inline]
    fn clone(&self) -> Tp27 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp27 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariableMultiple
pub struct Tp28;
#[automatically_derived]
impl ::core::clone::Clone for Tp28 {
    #[inline]
    fn clone(&self) -> Tp28 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp28 {}
#[must_use]
///BText
pub struct Tp29;
#[automatically_derived]
impl ::core::clone::Clone for Tp29 {
    #[inline]
    fn clone(&self) -> Tp29 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp29 {}
#[must_use]
///CElementWithAttribute
pub struct Tp30;
#[automatically_derived]
impl ::core::clone::Clone for Tp30 {
    #[inline]
    fn clone(&self) -> Tp30 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp30 {}
#[must_use]
///CElementWithContent
pub struct Tp31;
#[automatically_derived]
impl ::core::clone::Clone for Tp31 {
    #[inline]
    fn clone(&self) -> Tp31 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp31 {}
#[must_use]
///CEmptyElement
pub struct Tp32;
#[automatically_derived]
impl ::core::clone::Clone for Tp32 {
    #[inline]
    fn clone(&self) -> Tp32 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp32 {}
#[must_use]
///CSelfClosing
pub struct Tp33;
#[automatically_derived]
impl ::core::clone::Clone for Tp33 {
    #[inline]
    fn clone(&self) -> Tp33 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp33 {}
#[must_use]
///CSelfClosingBooleanAttr
pub struct Tp34;
#[automatically_derived]
impl ::core::clone::Clone for Tp34 {
    #[inline]
    fn clone(&self) -> Tp34 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp34 {}
#[must_use]
///DElementWithAttributeAndVariables
pub struct Tp35;
#[automatically_derived]
impl ::core::clone::Clone for Tp35 {
    #[inline]
    fn clone(&self) -> Tp35 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp35 {}
#[must_use]
///DElementWithAttributeAndVariables
pub struct Tp36;
#[automatically_derived]
impl ::core::clone::Clone for Tp36 {
    #[inline]
    fn clone(&self) -> Tp36 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp36 {}
#[must_use]
///DVariable
pub struct Tp37;
#[automatically_derived]
impl ::core::clone::Clone for Tp37 {
    #[inline]
    fn clone(&self) -> Tp37 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp37 {}
#[must_use]
///EIfElse
pub struct Tp38;
#[automatically_derived]
impl ::core::clone::Clone for Tp38 {
    #[inline]
    fn clone(&self) -> Tp38 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp38 {}
#[must_use]
///EIfElseEmpty
pub struct Tp39;
#[automatically_derived]
impl ::core::clone::Clone for Tp39 {
    #[inline]
    fn clone(&self) -> Tp39 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp39 {}
#[must_use]
///EIfElseEmptyFalse
pub struct Tp40;
#[automatically_derived]
impl ::core::clone::Clone for Tp40 {
    #[inline]
    fn clone(&self) -> Tp40 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp40 {}
#[must_use]
///EIfElseEmptyTrue
pub struct Tp41;
#[automatically_derived]
impl ::core::clone::Clone for Tp41 {
    #[inline]
    fn clone(&self) -> Tp41 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp41 {}
#[must_use]
///EIfElseWithVariables
pub struct Tp42;
#[automatically_derived]
impl ::core::clone::Clone for Tp42 {
    #[inline]
    fn clone(&self) -> Tp42 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp42 {}
#[must_use]
///EIfElseWithVariables
pub struct Tp43;
#[automatically_derived]
impl ::core::clone::Clone for Tp43 {
    #[inline]
    fn clone(&self) -> Tp43 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp43 {}
#[must_use]
///FEachEmpty
pub struct Tp44;
#[automatically_derived]
impl ::core::clone::Clone for Tp44 {
    #[inline]
    fn clone(&self) -> Tp44 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp44 {}
#[must_use]
///FEachOneVariable
pub struct Tp45;
#[automatically_derived]
impl ::core::clone::Clone for Tp45 {
    #[inline]
    fn clone(&self) -> Tp45 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp45 {}
#[must_use]
///FEachOneVariable
pub struct Tp46;
#[automatically_derived]
impl ::core::clone::Clone for Tp46 {
    #[inline]
    fn clone(&self) -> Tp46 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp46 {}
#[must_use]
///FEachTwoVariables
pub struct Tp47;
#[automatically_derived]
impl ::core::clone::Clone for Tp47 {
    #[inline]
    fn clone(&self) -> Tp47 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp47 {}
#[must_use]
///FEachTwoVariables
pub struct Tp48;
#[automatically_derived]
impl ::core::clone::Clone for Tp48 {
    #[inline]
    fn clone(&self) -> Tp48 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp48 {}
#[must_use]
///FEachTwoVariables
pub struct Tp49;
#[automatically_derived]
impl ::core::clone::Clone for Tp49 {
    #[inline]
    fn clone(&self) -> Tp49 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp49 {}
#[must_use]
///FEachTwoVariablesHtml
pub struct Tp50;
#[automatically_derived]
impl ::core::clone::Clone for Tp50 {
    #[inline]
    fn clone(&self) -> Tp50 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp50 {}
#[must_use]
///FEachTwoVariablesHtml
pub struct Tp51;
#[automatically_derived]
impl ::core::clone::Clone for Tp51 {
    #[inline]
    fn clone(&self) -> Tp51 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp51 {}
#[must_use]
///GEmptyTemplate
pub struct Tp52;
#[automatically_derived]
impl ::core::clone::Clone for Tp52 {
    #[inline]
    fn clone(&self) -> Tp52 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp52 {}
#[must_use]
///GEmptyTemplate
pub struct Tp53;
#[automatically_derived]
impl ::core::clone::Clone for Tp53 {
    #[inline]
    fn clone(&self) -> Tp53 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp53 {}
#[must_use]
///GEmptyTemplateMultiple
pub struct Tp54;
#[automatically_derived]
impl ::core::clone::Clone for Tp54 {
    #[inline]
    fn clone(&self) -> Tp54 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp54 {}
#[must_use]
///GEmptyTemplateMultiple
pub struct Tp55;
#[automatically_derived]
impl ::core::clone::Clone for Tp55 {
    #[inline]
    fn clone(&self) -> Tp55 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp55 {}
#[must_use]
///GEmptyTemplateMultiple
pub struct Tp56;
#[automatically_derived]
impl ::core::clone::Clone for Tp56 {
    #[inline]
    fn clone(&self) -> Tp56 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp56 {}
#[must_use]
///GEmptyTemplateMultiple
pub struct Tp57;
#[automatically_derived]
impl ::core::clone::Clone for Tp57 {
    #[inline]
    fn clone(&self) -> Tp57 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp57 {}
#[must_use]
///GOnlyPartialBlock
pub struct Tp58;
#[automatically_derived]
impl ::core::clone::Clone for Tp58 {
    #[inline]
    fn clone(&self) -> Tp58 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp58 {}
#[must_use]
///GPartialBlock
pub struct Tp59;
#[automatically_derived]
impl ::core::clone::Clone for Tp59 {
    #[inline]
    fn clone(&self) -> Tp59 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp59 {}
#[must_use]
///GPartialBlock
pub struct Tp60;
#[automatically_derived]
impl ::core::clone::Clone for Tp60 {
    #[inline]
    fn clone(&self) -> Tp60 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp60 {}
#[must_use]
///GPartialBlock
pub struct Tp61;
#[automatically_derived]
impl ::core::clone::Clone for Tp61 {
    #[inline]
    fn clone(&self) -> Tp61 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp61 {}
#[must_use]
///GPartialBlock
pub struct Tp62;
#[automatically_derived]
impl ::core::clone::Clone for Tp62 {
    #[inline]
    fn clone(&self) -> Tp62 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp62 {}
#[must_use]
///GPartialBlockPartial
pub struct Tp63;
#[automatically_derived]
impl ::core::clone::Clone for Tp63 {
    #[inline]
    fn clone(&self) -> Tp63 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp63 {}
#[must_use]
///GPartialBlockPartial
pub struct Tp64;
#[automatically_derived]
impl ::core::clone::Clone for Tp64 {
    #[inline]
    fn clone(&self) -> Tp64 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp64 {}
#[must_use]
///GPartialBlockPartial
pub struct Tp65;
#[automatically_derived]
impl ::core::clone::Clone for Tp65 {
    #[inline]
    fn clone(&self) -> Tp65 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp65 {}
#[must_use]
///GTemplateOnlyPartialBlock
pub struct Tp66;
#[automatically_derived]
impl ::core::clone::Clone for Tp66 {
    #[inline]
    fn clone(&self) -> Tp66 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp66 {}
#[must_use]
///GTemplateOnlyPartialBlock
pub struct Tp67;
#[automatically_derived]
impl ::core::clone::Clone for Tp67 {
    #[inline]
    fn clone(&self) -> Tp67 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp67 {}
#[must_use]
///GTemplateOnlyPartialBlockMultiple
pub struct Tp68;
#[automatically_derived]
impl ::core::clone::Clone for Tp68 {
    #[inline]
    fn clone(&self) -> Tp68 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp68 {}
#[must_use]
///GTemplateOnlyPartialBlockMultiple
pub struct Tp69;
#[automatically_derived]
impl ::core::clone::Clone for Tp69 {
    #[inline]
    fn clone(&self) -> Tp69 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp69 {}
#[must_use]
///GTemplateOnlyPartialBlockMultiple
pub struct Tp70;
#[automatically_derived]
impl ::core::clone::Clone for Tp70 {
    #[inline]
    fn clone(&self) -> Tp70 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp70 {}
#[must_use]
///GTemplateOnlyPartialBlockMultiple
pub struct Tp71;
#[automatically_derived]
impl ::core::clone::Clone for Tp71 {
    #[inline]
    fn clone(&self) -> Tp71 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp71 {}
#[must_use]
///GTemplateOnlyPartialBlockWithText
pub struct Tp72;
#[automatically_derived]
impl ::core::clone::Clone for Tp72 {
    #[inline]
    fn clone(&self) -> Tp72 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp72 {}
#[must_use]
///GTemplateOnlyPartialBlockWithText
pub struct Tp73;
#[automatically_derived]
impl ::core::clone::Clone for Tp73 {
    #[inline]
    fn clone(&self) -> Tp73 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp73 {}
#[must_use]
///GTemplateOnlyPartialBlockWithText
pub struct Tp74;
#[automatically_derived]
impl ::core::clone::Clone for Tp74 {
    #[inline]
    fn clone(&self) -> Tp74 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp74 {}
#[must_use]
///GTemplateOnlyPartialBlockWithTextMultiple
pub struct Tp75;
#[automatically_derived]
impl ::core::clone::Clone for Tp75 {
    #[inline]
    fn clone(&self) -> Tp75 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp75 {}
#[must_use]
///GTemplateOnlyPartialBlockWithTextMultiple
pub struct Tp76;
#[automatically_derived]
impl ::core::clone::Clone for Tp76 {
    #[inline]
    fn clone(&self) -> Tp76 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp76 {}
#[must_use]
///GTemplateOnlyPartialBlockWithTextMultiple
pub struct Tp77;
#[automatically_derived]
impl ::core::clone::Clone for Tp77 {
    #[inline]
    fn clone(&self) -> Tp77 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp77 {}
#[must_use]
///GTemplateOnlyPartialBlockWithTextMultiple
pub struct Tp78;
#[automatically_derived]
impl ::core::clone::Clone for Tp78 {
    #[inline]
    fn clone(&self) -> Tp78 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp78 {}
#[must_use]
///GTemplateOnlyPartialBlockWithTextMultiple
pub struct Tp79;
#[automatically_derived]
impl ::core::clone::Clone for Tp79 {
    #[inline]
    fn clone(&self) -> Tp79 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp79 {}
#[must_use]
///GTemplateOnlyPartialBlockWithTextMultiple
pub struct Tp80;
#[automatically_derived]
impl ::core::clone::Clone for Tp80 {
    #[inline]
    fn clone(&self) -> Tp80 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp80 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariable
pub struct Tp81;
#[automatically_derived]
impl ::core::clone::Clone for Tp81 {
    #[inline]
    fn clone(&self) -> Tp81 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp81 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariable
pub struct Tp82;
#[automatically_derived]
impl ::core::clone::Clone for Tp82 {
    #[inline]
    fn clone(&self) -> Tp82 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp82 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariable
pub struct Tp83;
#[automatically_derived]
impl ::core::clone::Clone for Tp83 {
    #[inline]
    fn clone(&self) -> Tp83 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp83 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariable
pub struct Tp84;
#[automatically_derived]
impl ::core::clone::Clone for Tp84 {
    #[inline]
    fn clone(&self) -> Tp84 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp84 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariableMultiple
pub struct Tp85;
#[automatically_derived]
impl ::core::clone::Clone for Tp85 {
    #[inline]
    fn clone(&self) -> Tp85 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp85 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariableMultiple
pub struct Tp86;
#[automatically_derived]
impl ::core::clone::Clone for Tp86 {
    #[inline]
    fn clone(&self) -> Tp86 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp86 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariableMultiple
pub struct Tp87;
#[automatically_derived]
impl ::core::clone::Clone for Tp87 {
    #[inline]
    fn clone(&self) -> Tp87 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp87 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariableMultiple
pub struct Tp88;
#[automatically_derived]
impl ::core::clone::Clone for Tp88 {
    #[inline]
    fn clone(&self) -> Tp88 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp88 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariableMultiple
pub struct Tp89;
#[automatically_derived]
impl ::core::clone::Clone for Tp89 {
    #[inline]
    fn clone(&self) -> Tp89 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp89 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariableMultiple
pub struct Tp90;
#[automatically_derived]
impl ::core::clone::Clone for Tp90 {
    #[inline]
    fn clone(&self) -> Tp90 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp90 {}
#[must_use]
///GTemplateOnlyPartialBlockWithVariableMultiple
pub struct Tp91;
#[automatically_derived]
impl ::core::clone::Clone for Tp91 {
    #[inline]
    fn clone(&self) -> Tp91 {
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for Tp91 {}
impl<Partial: Copy, After> Tp<Tp1, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("hello".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp2, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static(
                "<a class=\"test\"></a>".as_bytes(),
            ))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp3, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<h1>hi</h1>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp4, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<h1></h1>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp5, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<!DOCTYPE>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp6, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<!DOCTYPE html>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp7, Partial, After> {
    pub async fn test(
        self,
        test: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Tp<Tp35, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<a class=\"".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_double_quoted_attribute(test))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("\">".as_bytes()))
            .await;
        Tp::<Tp35, Partial, After> {
            r#type: Tp35,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp35, Partial, After> {
    pub async fn var(self, var: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(var))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</a>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp8, Partial, After> {
    pub async fn test(self, test: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<p>".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(test))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</p>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp9, Partial, After> {
    pub async fn next_author_false(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("false".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp9, Partial, After> {
    pub async fn next_author_true(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("true".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp10, Partial, After> {
    pub async fn next_author_false(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp10, Partial, After> {
    pub async fn next_author_true(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp11, Partial, After> {
    pub async fn next_author_false(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp11, Partial, After> {
    pub async fn next_author_true(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("true".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp12, Partial, After> {
    pub async fn next_author_false(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("false".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp12, Partial, After> {
    pub async fn next_author_true(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp13, Partial, After> {
    pub async fn next(self) -> Tp<Tp42, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<span>".as_bytes()))
            .await;
        Tp::<Tp42, Partial, After> {
            r#type: Tp42,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp42, Partial, After> {
    pub async fn f_author_false(self, f: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(f))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</span>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp42, Partial, After> {
    pub async fn t_author_true(self, t: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(t))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</span>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp14, Partial, After> {
    pub async fn next_enter_loop(self) -> Tp<Tp14, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp14, Partial, After> {
            r#type: Tp14,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp14, Partial, After> {
    pub async fn next_end_loop(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp15, Partial, After> {
    pub async fn next(self) -> Tp<Tp45, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<span>".as_bytes()))
            .await;
        Tp::<Tp45, Partial, After> {
            r#type: Tp45,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp45, Partial, After> {
    pub async fn title_enter_loop(
        self,
        title: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Tp<Tp45, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(title))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp45, Partial, After> {
            r#type: Tp45,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp45, Partial, After> {
    pub async fn next_end_loop(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</span>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp16, Partial, After> {
    pub async fn next(self) -> Tp<Tp47, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<span>".as_bytes()))
            .await;
        Tp::<Tp47, Partial, After> {
            r#type: Tp47,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp47, Partial, After> {
    pub async fn title_enter_loop(
        self,
        title: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Tp<Tp48, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(title))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp48, Partial, After> {
            r#type: Tp48,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp48, Partial, After> {
    pub async fn content(
        self,
        content: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Tp<Tp47, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(content))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp47, Partial, After> {
            r#type: Tp47,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp47, Partial, After> {
    pub async fn next_end_loop(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</span>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp17, Partial, After> {
    pub async fn title_enter_loop(
        self,
        title: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Tp<Tp50, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("\n    <li>".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(title))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</li>\n    <li>".as_bytes()))
            .await;
        Tp::<Tp50, Partial, After> {
            r#type: Tp50,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp50, Partial, After> {
    pub async fn content(
        self,
        content: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Tp<Tp17, Partial, After> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(content))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</li>\n".as_bytes()))
            .await;
        Tp::<Tp17, Partial, After> {
            r#type: Tp17,
            partial: self.partial,
            after: self.after,
        }
    }
}
impl<Partial: Copy, After> Tp<Tp17, Partial, After> {
    pub async fn next_end_loop(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp18, Partial, After> {
    pub async fn next(self) -> Tp<Tp52, Partial, ()> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp52, Partial, ()> {
            r#type: Tp52,
            partial: self.partial,
            after: (),
        }
    }
}
impl<Partial: Copy, After> Tp<Tp19, Partial, After> {
    pub async fn next(self) -> Tp<Tp54, Partial, ()> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp54, Partial, ()> {
            r#type: Tp54,
            partial: self.partial,
            after: (),
        }
    }
}
impl<Partial: Copy, After> Tp<Tp54, Partial, After> {
    pub async fn next(self) -> Tp<Tp56, Partial, ()> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp56, Partial, ()> {
            r#type: Tp56,
            partial: self.partial,
            after: (),
        }
    }
}
impl<PartialName: Copy, PartialPartial, PartialAfter, After>
    Tp<Tp20, Tp<PartialName, PartialPartial, PartialAfter>, After>
{
    pub async fn next(
        self,
    ) -> Tp<PartialName, (), Tp<Tp58, Tp<PartialName, PartialPartial, PartialAfter>, After>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<PartialName, (), Tp<Tp58, Tp<PartialName, PartialPartial, PartialAfter>, After>> {
            r#type: self.partial.r#type,
            partial: (),
            after: Tp::<Tp58, Tp<PartialName, PartialPartial, PartialAfter>, After> {
                r#type: Tp58,
                partial: self.partial,
                after: self.after,
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp21, Partial, After> {
    pub async fn next(
        self,
    ) -> Tp<Tp22, Tp<Tp60, (), Tp<Tp59, Partial, ()>>, Tp<Tp59, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<span>hello".as_bytes()))
            .await;
        Tp::<Tp22, Tp<Tp60, (), Tp<Tp59, Partial, ()>>, Tp<Tp59, Partial, ()>> {
            r#type: Tp22,
            partial: Tp::<Tp60, (), Tp<Tp59, Partial, ()>> {
                r#type: Tp60,
                partial: (),
                after: Tp::<Tp59, Partial, ()> {
                    r#type: Tp59,
                    partial: self.partial,
                    after: (),
                },
            },
            after: Tp::<Tp59, Partial, ()> {
                r#type: Tp59,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp60, Partial, After> {
    pub async fn test(self, test: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("childrenstart".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(test))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("childrenend".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp59, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("world</span>".as_bytes()))
            .await;
        self.after
    }
}
impl<PartialName: Copy, PartialPartial, PartialAfter, After>
    Tp<Tp22, Tp<PartialName, PartialPartial, PartialAfter>, After>
{
    pub async fn before(
        self,
        before: impl Into<::alloc::borrow::Cow<'static, str>>,
    ) -> Tp<PartialName, (), Tp<Tp63, Tp<PartialName, PartialPartial, PartialAfter>, After>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<span>".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(before))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<p>".as_bytes()))
            .await;
        Tp::<PartialName, (), Tp<Tp63, Tp<PartialName, PartialPartial, PartialAfter>, After>> {
            r#type: self.partial.r#type,
            partial: (),
            after: Tp::<Tp63, Tp<PartialName, PartialPartial, PartialAfter>, After> {
                r#type: Tp63,
                partial: self.partial,
                after: self.after,
            },
        }
    }
}
impl<PartialName: Copy, PartialPartial, PartialAfter, After>
    Tp<Tp63, Tp<PartialName, PartialPartial, PartialAfter>, After>
{
    pub async fn next(
        self,
    ) -> Tp<PartialName, (), Tp<Tp64, Tp<PartialName, PartialPartial, PartialAfter>, After>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</p><div>".as_bytes()))
            .await;
        Tp::<PartialName, (), Tp<Tp64, Tp<PartialName, PartialPartial, PartialAfter>, After>> {
            r#type: self.partial.r#type,
            partial: (),
            after: Tp::<Tp64, Tp<PartialName, PartialPartial, PartialAfter>, After> {
                r#type: Tp64,
                partial: self.partial,
                after: self.after,
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp64, Partial, After> {
    pub async fn after(self, after: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</div>".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(after))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</span>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp23, Partial, After> {
    pub async fn next(self) -> Tp<Tp20, Tp<Tp66, Partial, ()>, Tp<Tp66, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp66, Partial, ()>, Tp<Tp66, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp66, Partial, ()> {
                r#type: Tp66,
                partial: self.partial,
                after: (),
            },
            after: Tp::<Tp66, Partial, ()> {
                r#type: Tp66,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp24, Partial, After> {
    pub async fn next(self) -> Tp<Tp20, Tp<Tp68, Partial, ()>, Tp<Tp68, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp68, Partial, ()>, Tp<Tp68, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp68, Partial, ()> {
                r#type: Tp68,
                partial: self.partial,
                after: (),
            },
            after: Tp::<Tp68, Partial, ()> {
                r#type: Tp68,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp68, Partial, After> {
    pub async fn next(self) -> Tp<Tp20, Tp<Tp70, Partial, ()>, Tp<Tp70, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp70, Partial, ()>, Tp<Tp70, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp70, Partial, ()> {
                r#type: Tp70,
                partial: self.partial,
                after: (),
            },
            after: Tp::<Tp70, Partial, ()> {
                r#type: Tp70,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp25, Partial, After> {
    pub async fn next(
        self,
    ) -> Tp<Tp20, Tp<Tp73, (), Tp<Tp72, Partial, ()>>, Tp<Tp72, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp73, (), Tp<Tp72, Partial, ()>>, Tp<Tp72, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp73, (), Tp<Tp72, Partial, ()>> {
                r#type: Tp73,
                partial: (),
                after: Tp::<Tp72, Partial, ()> {
                    r#type: Tp72,
                    partial: self.partial,
                    after: (),
                },
            },
            after: Tp::<Tp72, Partial, ()> {
                r#type: Tp72,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp73, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("test".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp26, Partial, After> {
    pub async fn next(
        self,
    ) -> Tp<Tp20, Tp<Tp76, (), Tp<Tp75, Partial, ()>>, Tp<Tp75, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp76, (), Tp<Tp75, Partial, ()>>, Tp<Tp75, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp76, (), Tp<Tp75, Partial, ()>> {
                r#type: Tp76,
                partial: (),
                after: Tp::<Tp75, Partial, ()> {
                    r#type: Tp75,
                    partial: self.partial,
                    after: (),
                },
            },
            after: Tp::<Tp75, Partial, ()> {
                r#type: Tp75,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp76, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("test".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp75, Partial, After> {
    pub async fn next(
        self,
    ) -> Tp<Tp20, Tp<Tp79, (), Tp<Tp78, Partial, ()>>, Tp<Tp78, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp79, (), Tp<Tp78, Partial, ()>>, Tp<Tp78, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp79, (), Tp<Tp78, Partial, ()>> {
                r#type: Tp79,
                partial: (),
                after: Tp::<Tp78, Partial, ()> {
                    r#type: Tp78,
                    partial: self.partial,
                    after: (),
                },
            },
            after: Tp::<Tp78, Partial, ()> {
                r#type: Tp78,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp79, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("test".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp27, Partial, After> {
    pub async fn next(
        self,
    ) -> Tp<Tp20, Tp<Tp82, (), Tp<Tp81, Partial, ()>>, Tp<Tp81, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<span>".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp82, (), Tp<Tp81, Partial, ()>>, Tp<Tp81, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp82, (), Tp<Tp81, Partial, ()>> {
                r#type: Tp82,
                partial: (),
                after: Tp::<Tp81, Partial, ()> {
                    r#type: Tp81,
                    partial: self.partial,
                    after: (),
                },
            },
            after: Tp::<Tp81, Partial, ()> {
                r#type: Tp81,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp82, Partial, After> {
    pub async fn test(self, test: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(test))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp81, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</span>".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp28, Partial, After> {
    pub async fn next(
        self,
    ) -> Tp<Tp20, Tp<Tp86, (), Tp<Tp85, Partial, ()>>, Tp<Tp85, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("<span>".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp86, (), Tp<Tp85, Partial, ()>>, Tp<Tp85, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp86, (), Tp<Tp85, Partial, ()>> {
                r#type: Tp86,
                partial: (),
                after: Tp::<Tp85, Partial, ()> {
                    r#type: Tp85,
                    partial: self.partial,
                    after: (),
                },
            },
            after: Tp::<Tp85, Partial, ()> {
                r#type: Tp85,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp86, Partial, After> {
    pub async fn test(self, test: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(test))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp85, Partial, After> {
    pub async fn next(
        self,
    ) -> Tp<Tp20, Tp<Tp89, (), Tp<Tp88, Partial, ()>>, Tp<Tp88, Partial, ()>> {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        Tp::<Tp20, Tp<Tp89, (), Tp<Tp88, Partial, ()>>, Tp<Tp88, Partial, ()>> {
            r#type: Tp20,
            partial: Tp::<Tp89, (), Tp<Tp88, Partial, ()>> {
                r#type: Tp89,
                partial: (),
                after: Tp::<Tp88, Partial, ()> {
                    r#type: Tp88,
                    partial: self.partial,
                    after: (),
                },
            },
            after: Tp::<Tp88, Partial, ()> {
                r#type: Tp88,
                partial: self.partial,
                after: (),
            },
        }
    }
}
impl<Partial: Copy, After> Tp<Tp89, Partial, After> {
    pub async fn test(self, test: impl Into<::alloc::borrow::Cow<'static, str>>) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(zero_cost_templating::encode_element_text(test))
            .await;
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("".as_bytes()))
            .await;
        self.after
    }
}
impl<Partial: Copy, After> Tp<Tp88, Partial, After> {
    pub async fn next(self) -> After {
        ::zero_cost_templating::FutureToStream(())
            ._yield(::bytes::Bytes::from_static("</span>".as_bytes()))
            .await;
        self.after
    }
}
#[allow(unused)]
/// Start
pub fn a_empty(stream: ::zero_cost_templating::FutureToStream) -> () {
    ()
}
const _a_empty_FORCE_RECOMPILE: &'static str = "";
#[allow(unused)]
/// Start
pub fn b_text(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp1, (), ()> {
    Tp::<Tp1, (), ()> {
        r#type: Tp1,
        partial: (),
        after: (),
    }
}
const _b_text_FORCE_RECOMPILE: &'static str = "hello";
#[allow(unused)]
/// Start
pub fn c_element_with_attribute(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp2, (), ()> {
    Tp::<Tp2, (), ()> {
        r#type: Tp2,
        partial: (),
        after: (),
    }
}
const _c_element_with_attribute_FORCE_RECOMPILE: &'static str = "<a class=\"test\"></a>";
#[allow(unused)]
/// Start
pub fn c_element_with_content(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp3, (), ()> {
    Tp::<Tp3, (), ()> {
        r#type: Tp3,
        partial: (),
        after: (),
    }
}
const _c_element_with_content_FORCE_RECOMPILE: &'static str = "<h1>hi</h1>";
#[allow(unused)]
/// Start
pub fn c_empty_element(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp4, (), ()> {
    Tp::<Tp4, (), ()> {
        r#type: Tp4,
        partial: (),
        after: (),
    }
}
const _c_empty_element_FORCE_RECOMPILE: &'static str = "<h1></h1>";
#[allow(unused)]
/// Start
pub fn c_self_closing(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp5, (), ()> {
    Tp::<Tp5, (), ()> {
        r#type: Tp5,
        partial: (),
        after: (),
    }
}
const _c_self_closing_FORCE_RECOMPILE: &'static str = "<!DOCTYPE>";
#[allow(unused)]
/// Start
pub fn c_self_closing_boolean_attr(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp6, (), ()> {
    Tp::<Tp6, (), ()> {
        r#type: Tp6,
        partial: (),
        after: (),
    }
}
const _c_self_closing_boolean_attr_FORCE_RECOMPILE: &'static str = "<!DOCTYPE html>";
#[allow(unused)]
/// Start
pub fn d_element_with_attribute_and_variables(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp7, (), ()> {
    Tp::<Tp7, (), ()> {
        r#type: Tp7,
        partial: (),
        after: (),
    }
}
const _d_element_with_attribute_and_variables_FORCE_RECOMPILE: &'static str =
    "<a class=\"{{test}}\">{{var}}</a>";
#[allow(unused)]
/// Start
pub fn d_variable(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp8, (), ()> {
    Tp::<Tp8, (), ()> {
        r#type: Tp8,
        partial: (),
        after: (),
    }
}
const _d_variable_FORCE_RECOMPILE: &'static str = "<p>{{test}}</p>";
#[allow(unused)]
/// Start
pub fn e_if_else(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp9, (), ()> {
    Tp::<Tp9, (), ()> {
        r#type: Tp9,
        partial: (),
        after: (),
    }
}
const _e_if_else_FORCE_RECOMPILE: &'static str = "{{#if author}}true{{else}}false{{/if}}";
#[allow(unused)]
/// Start
pub fn e_if_else_empty(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp10, (), ()> {
    Tp::<Tp10, (), ()> {
        r#type: Tp10,
        partial: (),
        after: (),
    }
}
const _e_if_else_empty_FORCE_RECOMPILE: &'static str = "{{#if author}}{{else}}{{/if}}";
#[allow(unused)]
/// Start
pub fn e_if_else_empty_false(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp11, (), ()> {
    Tp::<Tp11, (), ()> {
        r#type: Tp11,
        partial: (),
        after: (),
    }
}
const _e_if_else_empty_false_FORCE_RECOMPILE: &'static str = "{{#if author}}true{{else}}{{/if}}";
#[allow(unused)]
/// Start
pub fn e_if_else_empty_true(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp12, (), ()> {
    Tp::<Tp12, (), ()> {
        r#type: Tp12,
        partial: (),
        after: (),
    }
}
const _e_if_else_empty_true_FORCE_RECOMPILE: &'static str = "{{#if author}}{{else}}false{{/if}}";
#[allow(unused)]
/// Start
pub fn e_if_else_with_variables(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp13, (), ()> {
    Tp::<Tp13, (), ()> {
        r#type: Tp13,
        partial: (),
        after: (),
    }
}
const _e_if_else_with_variables_FORCE_RECOMPILE: &'static str =
    "<span>{{#if author}}{{t}}{{else}}{{f}}{{/if}}</span>";
#[allow(unused)]
/// Start
pub fn f_each_empty(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp14, (), ()> {
    Tp::<Tp14, (), ()> {
        r#type: Tp14,
        partial: (),
        after: (),
    }
}
const _f_each_empty_FORCE_RECOMPILE: &'static str = "{{#each articles}}{{/each}}";
#[allow(unused)]
/// Start
pub fn f_each_one_variable(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp15, (), ()> {
    Tp::<Tp15, (), ()> {
        r#type: Tp15,
        partial: (),
        after: (),
    }
}
const _f_each_one_variable_FORCE_RECOMPILE: &'static str =
    "<span>{{#each articles}}{{title}}{{/each}}</span>";
#[allow(unused)]
/// Start
pub fn f_each_two_variables(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp16, (), ()> {
    Tp::<Tp16, (), ()> {
        r#type: Tp16,
        partial: (),
        after: (),
    }
}
const _f_each_two_variables_FORCE_RECOMPILE: &'static str =
    "<span>{{#each articles}}{{title}}{{content}}{{/each}}</span>";
#[allow(unused)]
/// Start
pub fn f_each_two_variables_html(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp17, (), ()> {
    Tp::<Tp17, (), ()> {
        r#type: Tp17,
        partial: (),
        after: (),
    }
}
const _f_each_two_variables_html_FORCE_RECOMPILE: &'static str =
    "{{#each articles}}\n    <li>{{title}}</li>\n    <li>{{content}}</li>\n{{/each}}";
#[allow(unused)]
/// Start
pub fn g_empty_template(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp18, (), ()> {
    Tp::<Tp18, (), ()> {
        r#type: Tp18,
        partial: (),
        after: (),
    }
}
const _g_empty_template_FORCE_RECOMPILE: &'static str = "{{#>a_empty}}{{/a_empty}}";
#[allow(unused)]
/// Start
pub fn g_empty_template_multiple(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp19, (), ()> {
    Tp::<Tp19, (), ()> {
        r#type: Tp19,
        partial: (),
        after: (),
    }
}
const _g_empty_template_multiple_FORCE_RECOMPILE: &'static str =
    "{{#>a_empty}}{{/a_empty}}{{#>a_empty}}{{/a_empty}}";
#[allow(unused)]
/// Start
pub fn g_only_partial_block(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp20, (), ()> {
    Tp::<Tp20, (), ()> {
        r#type: Tp20,
        partial: (),
        after: (),
    }
}
const _g_only_partial_block_FORCE_RECOMPILE: &'static str = "{{>@partial-block}}";
#[allow(unused)]
/// Start
pub fn g_partial_block(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp21, (), ()> {
    Tp::<Tp21, (), ()> {
        r#type: Tp21,
        partial: (),
        after: (),
    }
}
const _g_partial_block_FORCE_RECOMPILE: &'static str = "<span>hello{{#>g_partial_block_partial}}childrenstart{{test}}childrenend{{/g_partial_block_partial}}world</span>";
#[allow(unused)]
/// Start
pub fn g_partial_block_partial(stream: ::zero_cost_templating::FutureToStream) -> Tp<Tp22, (), ()> {
    Tp::<Tp22, (), ()> {
        r#type: Tp22,
        partial: (),
        after: (),
    }
}
const _g_partial_block_partial_FORCE_RECOMPILE: &'static str =
    "<span>{{before}}<p>{{>@partial-block}}</p><div>{{>@partial-block}}</div>{{after}}</span>";
#[allow(unused)]
/// Start
pub fn g_template_only_partial_block(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp23, (), ()> {
    Tp::<Tp23, (), ()> {
        r#type: Tp23,
        partial: (),
        after: (),
    }
}
const _g_template_only_partial_block_FORCE_RECOMPILE: &'static str =
    "{{#>g_only_partial_block}}{{/g_only_partial_block}}";
#[allow(unused)]
/// Start
pub fn g_template_only_partial_block_multiple(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp24, (), ()> {
    Tp::<Tp24, (), ()> {
        r#type: Tp24,
        partial: (),
        after: (),
    }
}
const _g_template_only_partial_block_multiple_FORCE_RECOMPILE: &'static str = "{{#>g_only_partial_block}}{{/g_only_partial_block}}{{#>g_only_partial_block}}{{/g_only_partial_block}}";
#[allow(unused)]
/// Start
pub fn g_template_only_partial_block_with_text(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp25, (), ()> {
    Tp::<Tp25, (), ()> {
        r#type: Tp25,
        partial: (),
        after: (),
    }
}
const _g_template_only_partial_block_with_text_FORCE_RECOMPILE: &'static str =
    "{{#>g_only_partial_block}}test{{/g_only_partial_block}}";
#[allow(unused)]
/// Start
pub fn g_template_only_partial_block_with_text_multiple(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp26, (), ()> {
    Tp::<Tp26, (), ()> {
        r#type: Tp26,
        partial: (),
        after: (),
    }
}
const _g_template_only_partial_block_with_text_multiple_FORCE_RECOMPILE: &'static str = "{{#>g_only_partial_block}}test{{/g_only_partial_block}}{{#>g_only_partial_block}}test{{/g_only_partial_block}}";
#[allow(unused)]
/// Start
pub fn g_template_only_partial_block_with_variable(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp27, (), ()> {
    Tp::<Tp27, (), ()> {
        r#type: Tp27,
        partial: (),
        after: (),
    }
}
const _g_template_only_partial_block_with_variable_FORCE_RECOMPILE: &'static str =
    "<span>{{#>g_only_partial_block}}{{test}}{{/g_only_partial_block}}</span>";
#[allow(unused)]
/// Start
pub fn g_template_only_partial_block_with_variable_multiple(
    stream: ::zero_cost_templating::FutureToStream,
) -> Tp<Tp28, (), ()> {
    Tp::<Tp28, (), ()> {
        r#type: Tp28,
        partial: (),
        after: (),
    }
}
const _g_template_only_partial_block_with_variable_multiple_FORCE_RECOMPILE: &'static str = "<span>{{#>g_only_partial_block}}{{test}}{{/g_only_partial_block}}{{#>g_only_partial_block}}{{test}}{{/g_only_partial_block}}</span>";
pub async fn test(stream: ::zero_cost_templating::FutureToStream) {
    let template = g_partial_block(stream);
    let template = template.next().await;
    let template = template.before("before").await;
    let template = inner(template).await;
    let template = template.next().await;
    let template = inner(template).await;
    let template = template.after("after").await;
    template.next().await;
}
pub fn main() {
    let body = async {
        let stream = TheStream::new(test);
        {
            ::std::io::_print(format_args!(
                "size of &str: {0}\n",
                std::mem::size_of::<&str>()
            ));
        };
        {
            ::std::io::_print(format_args!(
                "size of Cow: {0}\n",
                std::mem::size_of::<Cow<'static, str>>(),
            ));
        };
        {
            ::std::io::_print(format_args!(
                "size of String: {0}\n",
                std::mem::size_of::<String>()
            ));
        };
        {
            ::std::io::_print(format_args!(
                "size of stream: {0}\n",
                std::mem::size_of_val(&stream)
            ));
        };
        let mut stream = ::core::pin::Pin::<&mut _> {
            pointer: &mut { stream },
        };
        let mut stdout = std::io::stdout().lock();
        while let Some(element) = stream.next().await {
            stdout.write_all(&element).unwrap();
        }
        stdout.write_all(b"\n").unwrap();
    };
    #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
    {
        return tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
            .block_on(body);
    }
}
