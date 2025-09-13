import {joiResolver} from "@hookform/resolvers/joi";
import {useForm} from "react-hook-form";
import {useContactUsFormSchema} from "./schemas/useContactUsFormSchema";
import {ContactUsType} from "../../types/ContactUsType";
import {GetElementValue} from "../../lib/page";
import {ContentFieldType} from "../../types/CmsPageType";
import {useContactUsForm} from "./hooks/useContactUsForm";
import { SentContactFormRequest } from "grpc-avored/cms_pb";

type ContactUsComponentProps = {
    contentFields: ContentFieldType[]
}
export const ContactSection = ((props: ContactUsComponentProps) => {
    const {
        register,
        handleSubmit,
        // formState: {}
    } = useForm<ContactUsType>({
        resolver: joiResolver(useContactUsFormSchema()),
    })
    const {mutate, data: ContactUsMutateResponse, isPending} = useContactUsForm();


    const submitHandler = async (data: ContactUsType) => {
        const request = new SentContactFormRequest();

        request.setFirstName(data.first_name)
        request.setLastName(data.last_name)
        request.setEmail(data.email)
        request.setPhone(data.phone)
        request.setMessage(data.message)

        mutate(request)
    }

    return (
        <div className="mt-10 px-5">
            <hr/>
            <div>
                {(ContactUsMutateResponse?.getStatus() === true) ? (
                    <div
                        className="mt-5 bg-primary-100 border border-primary-200 text-sm text-primary-800 rounded-lg p-4"
                        role="alert">
                        <span className="font-bold">Success</span>
                        <span className="ml-3">
                            Contact form submitted successfully. We will get back to you soon.
                        </span>
                    </div>
                ) : ''}
            </div>

            <div className="mb-10 block">
                <div className="text-4xl heading-font text-primary-700 font-bold mt-10">
                    Contact us
                </div>
                <div className="md:flex flex-row pt-10">
                    <div className="md:w-1/2 w-full pr-5">
                        <div className="text-xl mb-3">
                            {GetElementValue(props.contentFields, 'contact-us-title')}
                        </div>
                        <div className="text-sm">
                            <p>
                                {GetElementValue(props.contentFields, 'contact-us-description')}
                            </p>
                        </div>
                    </div>
                    <div className="md:w-1/2 mt-10 md:mt-1 w-full pl-5">
                        <div className="text-2xl">
                            Talk with us today about your business
                        </div>
                        <form method="post" onSubmit={handleSubmit(submitHandler)}>
                            <div className="contact-form">

                                    <div className="md:flex block mt-3">
                                        <div className="md:w-1/2 w-full md:pr-3">
                                            <input
                                                aria-label="First Name"
                                                {...register('first_name')}
                                                type="text"
                                                required
                                                className="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                                placeholder="First Name"
                                            />
                                        </div>
                                        <div className="md:w-1/2 mt-3 md:mt-0 w-full md:pl-3">
                                            <input
                                                aria-label="Last Name"
                                                {...register('last_name')}
                                                type="text"
                                                className="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                                placeholder="Last Name"
                                            />
                                        </div>
                                    </div>
                                    <div className="md:flex block mt-3">
                                        <div className="md:w-1/2 w-full mt-3 md:pr-3">
                                            <input
                                                aria-label="Email address"
                                                {...register('email')}
                                                type="email"
                                                required
                                                className="appearance-none  relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                                placeholder="Email address"
                                            />
                                        </div>
                                        <div className="md:w-1/2 w-full mt-3 md:pl-3">
                                            <input
                                                aria-label="Phone"
                                                {...register('phone')}
                                                name="phone"
                                                type="text"
                                                className="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                                placeholder="Phone"
                                            />
                                        </div>
                                    </div>
                                    <div className="mt-3">
                                      <textarea
                                          {...register('message')}
                                          placeholder="Your Message"
                                          className="appearance-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                      ></textarea>
                                    </div>
                                    <div className="mt-6">
                                        <button
                                            type="submit"
                                            className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded text-white bg-primary-600 hover:bg-primary-500 focus:outline-none focus:border-primary-700 focus:shadow-outline-primary active:bg-primary-700"
                                        >
                                            {isPending ? 'Sending' : 'SENT'}
                                        </button>
                                    </div>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    )
})
