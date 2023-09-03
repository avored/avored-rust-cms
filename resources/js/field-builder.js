export default () => ({
    data: [],
    init() {
        console.log("field builder init")
    },

    addFieldButtonOnClick() { 
        console.log('identifier', new Date().getTime())
        this.data.push({identifier: new Date().getTime()})
    },
    setFieldType(event, identifier) {
      let field = this.data.find(x => x.identifier === identifier);
      field.type = event.target.value
    },

  });
  