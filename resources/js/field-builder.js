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
    setFieldNameValue(event, identifier) {
      let field = this.data.find(x => x.identifier === identifier);
      field.name_value = event.target.value
    },
    setFieldIdentifierValue(event, identifier) {
      let field = this.data.find(x => x.identifier === identifier);
      field.identifier_value = event.target.value
    },
    getItemNameValue() {
      let nameValues = [];
      this.data.map(x =>  {
        nameValues.push(x.name_value)
      })

      return nameValues.join(',')
    },
    getItemIdentifierValue() {
        let identifierValues = [];
        this.data.map(x =>  {
          identifierValues.push(x.identifier_value)
        })

        return identifierValues.join(',')
    },
  });
  