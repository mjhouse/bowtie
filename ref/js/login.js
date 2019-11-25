
let registerTab = document.getElementById("register-tab-label");
let loginTab = document.getElementById("login-tab-label");

let registerForm = document.getElementById("auth-register-tab");
let loginForm = document.getElementById("auth-login-tab");

registerForm.style.display = "none";

registerTab.onclick = function(){
    loginTab.classList.remove("auth-tab-active");
    registerTab.classList.add("auth-tab-active");

    registerForm.style.display = "table-row";
    loginForm.style.display = "none";
}

loginTab.onclick = function(){
    registerTab.classList.remove("auth-tab-active");
    loginTab.classList.add("auth-tab-active");

    registerForm.style.display = "none";
    loginForm.style.display = "table-row";
}
