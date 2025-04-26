export type AssetType = {
    id: string;
    name: string;
    new_path: string;
    asset_type: string;
    metadata: string;
    created_at: string;
    created_by: string;
    updated_at: string;
    updated_by: string;
}

export type AssetSaveType = {
    file_list?: FileList;
    file?: File;
}
export type CreateFolderType = {
    name: string;
    parent_id?: string;
}

export type DeleteFolderType = {
    asset_id: String;
}

export type RenameFolderType = {
    id: string;
    name: String;
}
