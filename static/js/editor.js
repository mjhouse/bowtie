function $(i) { return document.getElementById(i); };
function _(a,b,c) { a.addEventListener(b,c); };
function k(e) { return (e.keyCode ? e.keyCode : e.which); }
function exec(e,f,c){
    _(f,"click",() => {
        document.execCommand(c);
        e.focus();        
    })
};

const KeyCode = {
    Shift: 16,
    Control: 17,
    B: 66,
    I: 73    
};

(( 
    // structure
    toolbar,
    element,
    field,
    
    // buttons
    italic,
    bold ) => {

    // make the javascript editor visible
    toolbar.style.display = "block";
    element.style.display = "block";
    field.style.display   = "none";

    let edit = {
        control: false,
        hotkeys: {},
        bindings: function(c) {
            for(var v in this.hotkeys){
                if(v == c) {
                    this.hotkeys[v].click();
                    return true; }
            }
            return false;
        },
        keyup: function(e) {
            let c = k(e);
            if(c == KeyCode.Control)
                this.control = false;
        },
        keydown: function(e) {
            let c = k(e);
            if(c == KeyCode.Control)
                this.control = true;
            else if(this.control && this.bindings(c))
                e.preventDefault();
        }
    };

    // bind buttons to execCommand values
    exec(element,italic,"italic");
    exec(element,bold,"bold");

    // bind hotkeys to buttons
    edit.hotkeys[KeyCode.I] = italic;
    edit.hotkeys[KeyCode.B] = bold;

    // set keyup/keydown to trigger hotkeys
    _(element,"keyup",(e) => edit.keyup(e));
    _(element,"keydown",(e) => edit.keydown(e));

    // transfer text to real form element
    // on submit
    _(field.form,"submit",() => {
        field.value = element.innerHTML;
        return true;
    })
})(
    // structure
    $("toolbar"),
    $("editor"),
    $("field"),
    
    // buttons
    $("italic"),
    $("bold")
);