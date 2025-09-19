export type AssetType = {
    id: string;
    name: string;
    newPath: string;
    assetType: string;
    metadata: string;
    createdAt: string;
    createdBy: string;
    updatedAt: string;
    updatedBy: string;
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
    name: string;
}
