import axios from "axios";

export default () => ({
    component_modal: false,
    selected_component_id: '',
    page_components: [],
    components: [],
    async init() {
        const result = await axios.get("/api/component-all")
        if (result.status === 200) {
            this.components = result.data
        }

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
    },
    add_component_onclick() {
        const selected_component = this.components.find((component) => component.id === this.selected_component_id)
        // console.log(selected_component.id, selected_component.length)
        this.page_components.push(selected_component)
        this.close_component_modal()
        console.log(this.page_components)
    }
  });
  