import _ from "lodash";

export const GetElementValue = ((component: any, element_identifier: string) => {
    const element = _.get(component, 'elements', []).find((ele: any) => {
        return ele.identifier === element_identifier
    })
    return _.get(element, 'element_content', '')
})