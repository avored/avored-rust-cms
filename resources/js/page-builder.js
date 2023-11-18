import axios from "axios";

export default () => ({
    component_modal: false,
    selected_component: '',
    components: [],
    async init(data) {
        const result = await axios.get("/api/component-all")
        if (result.status === 200) {
            this.components = result.data
        }

        console.log(this.components)


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
        console.log(this.components, this.selected_component)
    }
  });
  