function show_corporate() {
    let corporate_membership = document.getElementById("corporate-sponsorship");
    corporate_membership.style.display = "block";
    
    let membership = document.getElementById("membership");
    membership.style.display = "none";

}

function show_membership() {
    let corporate_membership = document.getElementById("corporate-sponsorship");
    corporate_membership.style.display = "none";
    
    let membership = document.getElementById("membership");
    membership.style.display = "block";
}