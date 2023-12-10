
#[must_use]
pub struct Template<Type, Partial, After> {
    pub r#type: Type,
    pub partial: Partial,
    pub after: After,
}

impl<Type, Partial, After> Template<Type, Partial, After> {
    pub fn map_inner<NewPartial, NewAfter>(
        self,
        new_partial: NewPartial,
        new_after: NewAfter,
    ) -> Template<Type, NewPartial, NewAfter> {
        Template {
            r#type: self.r#type,
            partial: new_partial,
            after: new_after,
        }
    }
}

#[must_use] pub struct TestTemplate0 ; #[must_use] pub struct TestTemplate1 ;
#[must_use] pub struct TestTemplate2 ; #[must_use] pub struct TestTemplate3 ;
#[must_use] pub struct TestTemplate4 ; #[must_use] pub struct TestTemplate5 ;
#[must_use] pub struct TestTemplate6 ; #[must_use] pub struct TestTemplate7 ;
#[allow(unused)] macro_rules! test_template0
{
    ($template : expr) =>
    {
        unreachable! () ; :: zero_cost_templating :: Template < TestTemplate1,
        _, _ >
        {
            partial_type : $template.partial_type, end_type :
            $template.end_type
        }
    }
} impl < Partial, After > Template < TestTemplate0, Partial, After >
{
    pub fn test_template0(template : Self) -> :: zero_cost_templating ::
    Template < TestTemplate1, Partial, After > { todo! () }
} #[allow(unused)] macro_rules! test_page_title1
{
    ($template : expr, $value : expr) =>
    {
        unreachable! () ; :: zero_cost_templating :: Template < TestTemplate2,
        _, _ >
        {
            partial_type : $template.partial_type, end_type :
            $template.end_type
        }
    }
} #[allow(unused)] macro_rules! test_csrf_token2
{
    ($template : expr, $value : expr) =>
    {
        unreachable! () ; :: zero_cost_templating :: Template < TestTemplate3,
        _, _ >
        {
            partial_type : $template.partial_type, end_type :
            $template.end_type
        }
    }
} #[allow(unused)] macro_rules! test_template3
{
    ($template : expr) =>
    {
        unreachable! () ; :: zero_cost_templating :: Template < TestTemplate4,
        _, _ >
        {
            partial_type : $template.partial_type, end_type :
            $template.end_type
        }
    }
} impl < Partial, After > Template < TestTemplate3, Partial, After >
{
    pub fn test_template3(template : Self) -> :: zero_cost_templating ::
    Template < TestTemplate4, Partial, After > { todo! () }
} #[allow(unused)] macro_rules! test_title4
{
    ($template : expr, $value : expr) =>
    {
        unreachable! () ; :: zero_cost_templating :: Template < TestTemplate5,
        _, _ >
        {
            partial_type : $template.partial_type, end_type :
            $template.end_type
        }
    }
} #[allow(unused)] macro_rules! test_text5
{
    ($template : expr, $value : expr) =>
    {
        unreachable! () ; :: zero_cost_templating :: Template < TestTemplate3,
        _, _ >
        {
            partial_type : $template.partial_type, end_type :
            $template.end_type
        }
    }
} #[allow(unused)] macro_rules! test_template6
{
    ($template : expr) =>
    {
        unreachable! () ; :: zero_cost_templating :: Template < TestTemplate6,
        _, _ >
        {
            partial_type : $template.partial_type, end_type :
            $template.end_type
        }
    }
} impl < Partial, After > Template < TestTemplate3, Partial, After >
{
    pub fn test_template6(template : Self) -> :: zero_cost_templating ::
    Template < TestTemplate6, Partial, After > { todo! () }
} #[allow(unused)] macro_rules! test_copyright_year7
{
    ($template : expr, $value : expr) =>
    { unreachable! () ; _magic_expression_result.end_type }
} #[allow(unused)] macro_rules! test_initial0
{
    () =>
    {
        unreachable! () ; :: zero_cost_templating :: Template < TestTemplate0,
        _, _ > { partial_type : (), end_type : () }
    }
} const _test_FORCE_RECOMPILE : & 'static str = include_str!
("/home/moritz/Documents/zero-cost-templating/zero-cost-templating/test.html.hbs")
;
#[:: futures_async_stream ::
stream(item = alloc :: borrow :: Cow < 'static, str >)] pub async fn test()
{
    let template =
    {
        :: zero_cost_templating :: Template < TestTemplate0, _, _ >
        { partial_type : (), end_type : () }
    } ; let template =
    {
        let _magic_expression_result : :: zero_cost_templating :: Template <
        TestTemplate0, _, _ > = template ; yield :: alloc :: borrow :: Cow ::
        from("<!DOCTYPE html>\n<html lang=\"en\">\n  <head>\n    <meta charset=\"utf-8\">\n    <title>title</title>\n    <link rel=\"stylesheet\" href=\"style.css\">\n    <script src=\"script.js\"></script>\n  </head>\n  <body>\n    <h1>")
        ; :: zero_cost_templating :: Template < TestTemplate1, _, _ >
        {
            partial_type : _magic_expression_result.partial_type, end_type :
            _magic_expression_result.end_type
        }
    } ; let page_title = Cow :: from("thetitle") ; let template =
    {
        let _magic_expression_result : :: zero_cost_templating :: Template <
        TestTemplate1, _, _ > = template ; yield zero_cost_templating ::
        encode_element_text(page_title) ; yield :: alloc :: borrow :: Cow ::
        from("</h1>\n    <input type=\"hidden\" value=\"") ; ::
        zero_cost_templating :: Template < TestTemplate2, _, _ >
        {
            partial_type : _magic_expression_result.partial_type, end_type :
            _magic_expression_result.end_type
        }
    } ; let csrf_token = Cow :: from("thetoken") ; let template =
    {
        let _magic_expression_result : :: zero_cost_templating :: Template <
        TestTemplate2, _, _ > = template ; yield zero_cost_templating ::
        encode_double_quoted_attribute(csrf_token) ; yield :: alloc :: borrow
        :: Cow :: from("\">\n    <ul>\n    ") ; :: zero_cost_templating ::
        Template < TestTemplate3, _, _ >
        {
            partial_type : _magic_expression_result.partial_type, end_type :
            _magic_expression_result.end_type
        }
    } ; let template =
    {
        let _magic_expression_result : :: zero_cost_templating :: Template <
        TestTemplate3, _, _ > = template ; yield :: alloc :: borrow :: Cow ::
        from("\n    </ul>\n    <span>") ; :: zero_cost_templating :: Template
        < TestTemplate6, _, _ >
        {
            partial_type : _magic_expression_result.partial_type, end_type :
            _magic_expression_result.end_type
        }
    } ;
    {
        let _magic_expression_result : :: zero_cost_templating :: Template <
        TestTemplate6, _, _ > = template ; yield zero_cost_templating ::
        encode_element_text(Cow :: from("2023".to_string())) ; yield :: alloc
        :: borrow :: Cow :: from("</span>\n  </body>\n</html>") ;
        _magic_expression_result.end_type
    } ;
}