export default () => ({

    init() {
        console.log("page builder")
        new EasyMDE({
            element: document.getElementById('easy_mde'),
            lineWrapping: true
        });
    },
  });
  