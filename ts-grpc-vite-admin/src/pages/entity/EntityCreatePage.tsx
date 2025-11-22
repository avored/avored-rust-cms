import InputField from "../../components/InputField";
import {useTranslation} from "react-i18next";
import ErrorMessage from "../../components/ErrorMessage";
import {Link} from "react-router-dom";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import _ from "lodash";
import { CreateEntityType } from "../../types/entity/EntityType";
import { UseEntityCreateSchema } from "../../schemas/entity/UseEntityCreateSchema";
import { StoreEntityRequest } from "grpc-avored/entity_pb";
import { UseStoreEntityHook } from "../../hooks/entity/UseStoreEntityHook";

export const EntityCreatePage = () => {

    const [t] = useTranslation("global")
    const {mutate, error} = UseStoreEntityHook()

    const {
        register,
        handleSubmit,
        formState: {errors},
    } = useForm<CreateEntityType>({
        resolver: joiResolver(UseEntityCreateSchema(), {allowUnknown: true, abortEarly: false})
    })

    const submitHandler = async (data: CreateEntityType) => {

        const store_entity = new StoreEntityRequest();

            store_entity.setName(data.name);
            store_entity.setIdentifier(data.identifier);
            mutate(store_entity)

    }

    return (
        <>
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("admin_user_information")}
                        </h1>
                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t("name")}
                                    type="text"
                                    name="name"
                                    register={register("name")}
                                    autoFocus
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="name" />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("identifier")}
                                    type="text"
                                    name="identifier"
                                    register={register("identifier")}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="identifier" />
                            </div>

                            
                            <div className="flex items-center mt-5">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to="/admin/entity"
                                    className="ml-auto font-medium text-gray-600 hover:text-gray-500"
                                >
                                    {t("cancel")}
                                </Link>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </>
    )
}
