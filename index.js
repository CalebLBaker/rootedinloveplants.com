$(document).ready(function() {
    var popups = document.getElementsByClassName("popup");
    var docElement = document.documentElement;
    var aspectRatio = docElement.clientWidth / docElement.clientHeight;
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
                description.style.width = `${width}vw`;
            }
        }
    }

    $("a.plant").fancybox();
});

