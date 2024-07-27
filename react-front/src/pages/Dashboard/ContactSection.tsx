import {joiResolver} from "@hookform/resolvers/joi";
import {useContactUsForm} from "./hooks/useContactUsForm.ts";
import {useForm} from "react-hook-form";
import {useContactUsFormSchema} from "./schemas/useContactUsFormSchema.ts";
import {ContactUsType} from "../../types/ContactUsType.ts";

export const ContactSection = (() => {
    const {
        register,
        handleSubmit,
        formState: {}
    } = useForm<ContactUsType>({
        resolver: joiResolver(useContactUsFormSchema()),
    })
    const {mutate, data: ContactUsMutateResponse, isPending} = useContactUsForm();
    const submitHandler = async (data:  ContactUsType) => {
       mutate(data)
    }

    return (
        <div className="mt-10 px-5">
            <hr/>
            <div>
                {(ContactUsMutateResponse?.data.status === true) ? (
                    <div
                        className="mt-5 bg-primary-100 border border-primary-200 text-sm text-primary-800 rounded-lg p-4"
                        role="alert">
                        <span className="font-bold">Success</span>
                        <span className="ml-3">
                            {ContactUsMutateResponse?.data.data.message}
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
                            Need help developing content management solution with Avoprimary?
                        </div>
                        <div className="text-sm">
                            <p>
                                We'd love to talk with you about your cms projects
                                whether it's a brand new website or a rework of your existing
                                one. We would happily answer any questions you may have about
                                developing with Avoprimary.
                            </p>
                            <p>
              <span className="font-semibold">
                Do not have a development team?
              </span>
                                Don't worry, we can implement the Avoprimary solution for you
                                according to your business requirements. We can surely help
                                you achieve your goals with a customized Avoprimary design.
                            </p>
                            <p>
              <span className="font-semibold">
                For a FREE consultation with our expert development team,
                simply leave your details below and we'll get back to you
                soon.
              </span>
                            </p>
                        </div>
                    </div>
                    <div className="md:w-1/2 mt-10 md:mt-1 w-full pl-5">
                        <div className="text-2xl">
                            Talk with us today about your business
                        </div>
                        <form method="post" onSubmit={handleSubmit(submitHandler)}>
                            <div className="contact-form">
                                <div className="rounded-md shadow-sm">
                                    <div className="flex mt-3">
                                        <div className="w-1/2 pr-3">
                                            <input
                                                aria-label="First Name"
                                                {...register('first_name')}
                                                type="text"
                                                required
                                                className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                                placeholder="First Name"
                                            />
                                        </div>
                                        <div className="w-1/2 pl-3">
                                            <input
                                                aria-label="Last Name"
                                                {...register('last_name')}
                                                type="text"
                                                className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                                placeholder="Last Name"
                                            />
                                        </div>
                                    </div>
                                    <div className="flex mt-3">
                                        <div className="w-1/2 pr-3">
                                            <input
                                                aria-label="Email address"
                                                {...register('email')}
                                                type="email"
                                                required
                                                className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                                placeholder="Email address"
                                            />
                                        </div>
                                        <div className="w-1/2 pl-3">
                                            <input
                                                aria-label="Phone"
                                                {...register('phone')}
                                                name="phone"
                                                type="text"
                                                className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                                                placeholder="Phone"
                                            />
                                        </div>
                                    </div>
                                    <div className="mt-3">
                  <textarea
                      {...register('message')}
                      placeholder="Your Message"
                      className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-b-md focus:outline-none focus:shadow-outline-primary focus:border-primary-300 focus:z-10 sm:text-sm sm:leading-5"
                  ></textarea>
                                    </div>
                                    <div className="mt-6">
                                        <button
                                            type="submit"
                                            className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-primary-600 hover:bg-primary-500 focus:outline-none focus:border-primary-700 focus:shadow-outline-primary active:bg-primary-700"
                                        >
                                            {isPending ? 'Sending' : 'SENT'}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </div>
    )
})
