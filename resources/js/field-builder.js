export default () => ({
    modal: false,
    field: {
        field_type: '',
        field_name: '',
        field_identifier: ''
    },
    fields: [],
    init() {
        console.log("field builder init")
    },

    fieldButtonOnClick(field_type) {
        this.field.field_type = field_type


    },
    feather_replace() {
        setTimeout(() => {
            window.feather.replace()
        }, 1000)
    },
    add_field_onclick() {
        this.fields.push(this.field)
        this.modal = ! this.modal
        this.feather_replace()
        console.log("test")
    },
    open_edit_modal(field) {
        this.modal = true
        this.field = field
    },
    close_modal() {
        this.resetFieldObject();
        this.modal = false
    },
    open_modal_button_onclick() {
        this.resetFieldObject()
        this.modal = true
    },
    resetFieldObject() {
        this.field = {}
    },
    fieldSelected() {
        console.log('close modal')
    },
    setFieldType(event, identifier) {

    },

  });
  