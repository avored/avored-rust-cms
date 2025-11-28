import {Link, useParams} from "react-router-dom";
import {useTranslation} from "react-i18next";
import {useState} from "react";
import {useForm} from "react-hook-form";
import {joiResolver} from "@hookform/resolvers/joi";
import InputField from "../../components/InputField";
import {UseEntityEditSchema} from "../../schemas/entity/UserEntityEditSchema";
import {UseGetEntityHook} from "../../hooks/entity/UseGetEntityHook";
import {EditEntityType, PutEntityIdentifierType} from "../../types/entity/EntityType";
import {UseEntityPutSchema} from "../../schemas/entity/UseEntityPutSchema";
import {UseUpdateEntityHook} from "../../hooks/entity/UseUpdateEntityHook";
import {UsePutEntityIdentifierHook} from "../../hooks/entity/UsePutEntityIdentifierHook";
import ErrorMessage from "../../components/ErrorMessage";
import { UseDeleteEntityHook } from "../../hooks/entity/UseDeleteEntityHook";
import { ButtonType } from "../../components/AvoRedButton";
import { ExclamationTriangleIcon } from "@heroicons/react/24/solid";
import AvoredModal from "../../components/AvoredModal";
import { GetEntityRequest, PutEntityIdentifierRequest, DeleteEntityRequest, UpdateEntityRequest } from "grpc-avored/entity_pb"

export const EntityEditPage = () => {
    const params = useParams();
    const [isEditableIdentifier, setIsEditableIdentifier] = useState<boolean>(true)
    const [isDeleteConfirmationModalOpen, setIiDeleteConfirmationModalOpen] = useState<boolean>(false)
    const entity_id = params.entity_id ?? ''

    const { mutate } = UseUpdateEntityHook();
    const [t] = useTranslation("global")

    const req = new GetEntityRequest();
    req.setEntityId(entity_id);

    const {data, error} = UseGetEntityHook(req)


    const values: EditEntityType = data?.data as unknown as EditEntityType;
    


    const {
        register: putEntityRegister,
        getValues: getEntityIdentifierValue
    } = useForm<PutEntityIdentifierType>({
        resolver: joiResolver(UseEntityPutSchema(), {allowUnknown: true}),
        values: {
            identifier: data?.data?.identifier ?? ''
        }
    });

    const {
        register,
        handleSubmit,
        formState: {errors},
    } = useForm<EditEntityType>({
        resolver: joiResolver(UseEntityEditSchema(), {allowUnknown: true}),
        values
    })

    const {mutate: putEntityIdentifierMutate} = UsePutEntityIdentifierHook()
    const {mutate: deleteEntityMutate} = UseDeleteEntityHook()


    const editableIdentifierOnClick = (() => {
        setIsEditableIdentifier(false)
    })
    const saveIdentifierOnClick = (() => {
        const putEntityIdentifier = new PutEntityIdentifierRequest();
        putEntityIdentifier.setEntityId(entity_id);
        const val = getEntityIdentifierValue('identifier') ?? '';
        putEntityIdentifier.setIdentifier(val.toString())

        putEntityIdentifierMutate(putEntityIdentifier)

        setIsEditableIdentifier(true)
    })

    const cancelIdentifierOnClick = (() => {
        setIsEditableIdentifier(true)
    })




    const submitHandler = ((data: EditEntityType) => {
        const update_entity = new UpdateEntityRequest();
        update_entity.setEntityId(data.id);
        update_entity.setName(data.name);

        mutate(update_entity)
    })



    const deleteButtonOnClick = (() => {
        setIiDeleteConfirmationModalOpen(true)
    })

    const confirmOnDelete = ((e: React.MouseEvent<HTMLButtonElement, MouseEvent>) => {
        e.preventDefault()
        const request = new DeleteEntityRequest()
        request.setEntityId(entity_id)

        
        deleteEntityMutate(request)        
    })

    return(
        <>
            <div className="px-5">
                <div className="w-full">
                    <div className="block rounded-lg p-6">
                        <h1 className="text-xl font-semibold mb-4 text-gray-900">
                            {t("entity_information")}
                        </h1>

                        <form onSubmit={handleSubmit(submitHandler)}>
                            <div className="mb-4">
                                <InputField
                                    label={t("name")}
                                    placeholder={t("name")}
                                    name="name"
                                    register={register("name")}
                                    autoFocus={true}
                                />
                                <ErrorMessage frontendErrors={errors} backendErrors={error} identifier="name" />
                            </div>
                            <div className="mb-4">
                                <InputField
                                    label={t("identifier")}
                                    placeholder={t("identifier")}
                                    name="identifier"
                                    register={putEntityRegister("identifier")}
                                    disabled={isEditableIdentifier}
                                />
                                <div
                                    className="mt-2"
                                >
                                    {isEditableIdentifier ? (
                                        <>
                                            <span onClick={editableIdentifierOnClick}
                                                  className="text-xs text-blue-600 cursor-pointer">
                                                {t("edit_identifier")}
                                            </span>
                                        </>
                                    ) : (
                                        <>
                                            <button type="button" onClick={saveIdentifierOnClick}
                                                    className="text-xs text-blue-600 cursor-pointer">
                                                {t('save')}
                                            </button>
                                            <button type="button" onClick={cancelIdentifierOnClick}
                                                    className="ml-3 text-xs text-blue-600 cursor-pointer">
                                                {t('cancel')}
                                            </button>
                                        </>
                                    )}
                                </div>
                            </div>
                        
                            <AvoredModal
                                isOpen={isDeleteConfirmationModalOpen}
                                
                                closeModal={() => setIiDeleteConfirmationModalOpen(false)}
                                modal_header=""
                                modal_body={
                                    <div>
                                        <div className="">
                                            <div className="p-6 pt-0 text-center">
                                                <ExclamationTriangleIcon className="w-20 h-20 text-red-600 mx-auto" />
                                                <h3 className="text-xl font-normal text-gray-500 mt-5 mb-6">
                                                    Are you sure you want to delete this entity?
                                                </h3>
                                                <button type="button"
                                                    onClick={e => confirmOnDelete(e)} 
                                                    className="text-white bg-red-600 hover:bg-red-800 focus:ring-4 focus:ring-red-300 font-medium rounded-lg text-base inline-flex items-center px-3 py-2.5 text-center mr-2">
                                                    Yes, I'm sure
                                                </button>
                                                <button type="button"
                                                    onClick={e => { e.preventDefault(); setIiDeleteConfirmationModalOpen(false)} } 
                                                    className="text-gray-900 bg-white hover:bg-gray-100 focus:ring-4 focus:ring-cyan-200 border border-gray-200 font-medium inline-flex items-center rounded-lg text-base px-3 py-2.5 text-center"
                                                    data-modal-toggle="delete-user-modal">
                                                    No, cancel
                                                </button>
                                            </div>
    
                                        </div>
                                    </div>
                                }
                            />


                            <div className="flex items-center">
                                <button
                                    type="submit"
                                    className="bg-primary-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-primary-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary-500"
                                >
                                    {t("save")}
                                </button>
                                <Link
                                    to="/admin/entity"
                                    className="ml-5 font-medium text-gray-600 hover:text-gray-500"
                                >
                                    {t("cancel")}
                                </Link>

                                <button
                                    onClick={deleteButtonOnClick}
                                    type={ButtonType.button}
                                    className="ml-auto bg-red-600 py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white hover:bg-red-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-red-500"
                                >
                                    {t("delete")}
                                </button>
                            </div>
                        </form>
                    </div>
                </div>
            </div>
        </>
    )
}
