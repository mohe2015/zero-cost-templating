Pretty surely we should use async generator functions as they are really nice to use. A good idea may even be to use a channel etc so pass the stream as argument?

* Templates engine loops should be Optimized If inner Body prints Something at Start and end
* Context specific Allowlist based escaping for maximum Performance
* Maybe proc macro template parsing If that can help us highlight errors (probably not)
* How do we pass the data in the most efficient way so it also keeps working when dynamically updating the template
* It also should correctly recompile when the template is updated
* It should be typesafe
* Currently needed primitives: output variable, loop over stream and provide variables in the scope inside (should also be provided step by step)
* ```
template: Initial = get_template(t).await;
template = yield template.csrf_token(token).await;
template = template.loop_item_start();
```
* Every call should provide a new value so efficiency is maximized
* The start directly prints everything until the first variable to get out as much data as early as possible
* All variable calls print out the (escaped) variable and known static string afterwards (in loops not possible, except maybe the last call could indicate it's the last value?) (Probably not useful as usually not known)
* `template = yield template.after_loop_date(date).await;` or `template = yield template.in_loop_username(username).await;`
* Next issue: method calls need to return new template type and value. Either pass writer to template (maybe in constructor) or return tuple?
* How to dynamically update template: the methods in dynamic mode iterate over the AST? We probably support strings only at the start anyways. Maybe integers too. Should be dynamic so you can create an escape safe value. Would be nice if we could Support async iterables (like large text) there too. Because escaping probably needs to handle that async. Then maybe pass the output to the template and don't use yield?
* Implement the dynamically update version first as we need that for development and it's probably still quite fast.
* If the variables and order didn't change the Optimized AST should (currently) look the same when the text changed so maybe we can store the AST as runtime value and just index into it in the generated methods. The compiler then generates a const value of that AST and then maybe it would be able to just inline the value and it would easily generate optimized Code
* We always have a common template like the outer html (that wants to adapt e.g. the title though). I think it would be helpful if you could write reusable code for that part. So maybe either reverse include and then add a generic parameter for what template comes afterwards (what about when this is in an inner template, does it work there?)
* Probably support both ways with generics
* We need to inline inner templates to maximize performance? Or somehow work around
* We probably don't want to support ifs but instead be able to optionally render something? Which usecases are there? Optional classes, ...