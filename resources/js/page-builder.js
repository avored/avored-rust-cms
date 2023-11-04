export default () => ({
    component_modal: false,
    components: [],
    init(data) {
        console.log(data)
        new EasyMDE({
            element: document.getElementById('easy_mde'),
            lineWrapping: true
        });
    },
    open_component_modal() {
        this.component_modal = true
    },
    close_component_modal() {
        this.component_modal = false
    }
  });
  