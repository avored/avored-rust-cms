import {DateTime} from "luxon";

const randomString = ((length: number = 16) => {
    let result = '';
    const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    const charactersLength = characters.length;
    let counter = 0;
    while (counter < length) {
        result += characters.charAt(Math.floor(Math.random() * charactersLength));
        counter += 1;
    }
    return result;
})

const getFormattedDate = ((date: string) => {
    let dateObject  = DateTime.fromJSDate(new Date(date))

    return dateObject.toLocaleString(DateTime.DATE_MED)
})
export {
    randomString,
    getFormattedDate
}