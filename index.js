// This is code I copy and pasted from Facebook's documentation.
(function(d, s, id) {
    var js, fjs = d.getElementsByTagName(s)[0];
    if (d.getElementById(id)) return;
    js = d.createElement(s); js.id = id;
    js.src = "https://connect.facebook.net/en_US/sdk/xfbml.customerchat.js#xfbml=1&version=v0.1&autoLogAppEvents=1";
    fjs.parentNode.insertBefore(js, fjs);
}(document, 'script', 'facebook-jssdk'));

var plantSizeFactor = 8.25;

function resizeStuff() {
    var docElement = document.documentElement;
    var aspectRatio = docElement.clientWidth / docElement.clientHeight;
    var plants = document.getElementsByClassName("plant");
    var popups = document.getElementsByClassName("popup");
    var plantWidth = (100.0 / Math.floor(Math.sqrt(aspectRatio) * plantSizeFactor)) + "%";
    for (var i = 0; i < plants.length; ++i) {
        plants.item(i).style.width = plantWidth;
    }
    for (var i = 0; i < popups.length; ++i) {
        var popup = popups.item(i);
        var pictures = popup.getElementsByClassName("picture");
        var descriptions = popup.getElementsByClassName("description");
        if (pictures.length > 0 && descriptions.length > 0) {
            var picture = pictures.item(0);
            var description = descriptions.item(0);
            var pictureAspectRatio = picture.naturalWidth / picture.naturalHeight;
            // Adding in an arbitrarily selected 0.5 multiplier
            // to compensate for the presence of other elements in the popup
            var popupAspectRatio = 0.5 * pictureAspectRatio;

            if (popupAspectRatio > aspectRatio) {
                // The popup is inclined to be wider than the screen. Make it not.
                picture.style.width = "80vw";
                description.style.width = "80vw";
            }
            else {
                // The popup is inclined to be taller than the screen. Make it not.
                picture.style.height = "40vh";
                var width = 80 * popupAspectRatio / aspectRatio;
                description.style.width = width + "vw";
            }
        }
    }
}

function setFontSize(tagName, fontSize) {
    var elements = document.getElementsByTagName(tagName);
    for (var i = 0; i < elements.length; ++i) {
        elements[i].style.fontSize = fontSize;
    }
}

$(document).ready(function() {
    $("a.plant").fancybox();

    // show the facebook messenger plugin
    FB.CustomerChat.show(true);

    // Make things slightly larger on mobile
    // This mobile detection isn't perfect, but the desktop version of the site doesn't look horrible
    // on mobile, nor does the mobile version look horrible on desktop, so it isn't the end of the world
    // if we get things wrong
    if (navigator.userAgent.match(/Android|BlackBerry|iPhone|iPad|iPod|Opera Mini|IEMobile/)) {
        setFontSize("p", "1.5rem");
        setFontSize("button", "1.5rem");
        setFontSize("input", "2.0rem");
        setFontSize("textarea", "2.0rem");
        setFontSize("h3", "1.7rem");
        setFontSize("h2", "2.0rem");
        plantSizeFactor = 4 * Math.sqrt(2);
    }

    resizeStuff();
    window.addEventListener("resize", resizeStuff);
});

function displayEmailForm(name) {
    document.getElementById("subject").value = "Interested in " + name;
    document.getElementById("emailLink").click();
}

function displayToast(id) {
    var toast = document.getElementById(id);
    toast.style.opacity = 1;
    setTimeout(function() { toast.style.opacity = 0; }, 2800);
}

function sendEmail() {
    var body = document.getElementById("body");
    var httpRequest = new XMLHttpRequest();

    httpRequest.onreadystatechange = (function() {
        if (httpRequest.readyState === XMLHttpRequest.DONE) {
            if (httpRequest.status === 200) {
                displayToast("successToast");
                body.value = "";
            } else {
                displayToast("errorToast");
            }
            $.fancybox.close();
        }
    });

    httpRequest.open("POST", "http://localhost:8080");
    httpRequest.send(JSON.stringify({
        "reply_to": document.getElementById("emailAddress").value,
        "subject": document.getElementById("subject").value,
        "body": body.value
    }));
}

