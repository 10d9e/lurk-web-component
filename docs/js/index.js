import init, {execute_lurk} from "./pkg/lurk_web.js";

init().then(() => {
    HighlightLisp.highlight_auto();

    // specify a custom class name (instead of "lisp"):
    //HighlightLisp.highlight_auto({className: 'common-lisp'});

    // highlight *every* code block
    //HighlightLisp.highlight_auto({className: null});

    // manually highlight a code block
    //var code = document.getElementById('my-code-element');
    //HighlightLisp.highlight_element(code);

    HighlightLisp.paren_match();

    //var btn = $(".button");
    var btn = document.getElementById('run');
    btn.onclick = function (e) {
        console.log("clicked");

        var output_container = document.getElementById("output-container");
        output_container.style.display = "block";

        var lurkcode = document.getElementById("lurkcode");
        
        console.log(lurkcode.textContent);
        var out = execute_lurk(lurkcode.textContent);
        var outObj = JSON.parse(out);
        console.log(outObj);
        var output = document.getElementById("output");
        output.textContent = "Iterations: " + outObj.iterations + "\nResult: " + outObj.result;

        return false;
    };

});

