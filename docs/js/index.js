import init, { execute_lurk, init_panic_hook } from "../pkg/lurk_web.js";

var originalContents;

init().then(() => {
    init_panic_hook();
    HighlightLisp.highlight_auto();
    HighlightLisp.paren_match();
    originalContents = document.getElementById("lurkcode").textContent;
    var btn = document.getElementById('run');
    btn.onclick = function (e) {
        var output_container = document.getElementById("output-container");
        output_container.style.display = "block";
        var lurkcode = document.getElementById("lurkcode");        
        var output = document.getElementById("output");
        try {
            var out = execute_lurk(lurkcode.textContent);
        } catch (error) {
            output.textContent ="Iterations: 0 \nResult: ERROR!";
            return false;
        }
        var outObj = JSON.parse(out);
        output.textContent = "Iterations: " + outObj.iterations + "\nResult: " + outObj.result;
        return false;
    };
    var resetBtn = document.getElementById('reset');
    resetBtn.onclick = function (e) {
        document.getElementById("lurkcode").textContent = originalContents;
        HighlightLisp.highlight_auto();
        HighlightLisp.paren_match();
        var output_container = document.getElementById("output-container");
        output_container.style.display = "none";
    }
});

