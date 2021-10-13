pub enum TypeCategory {
    Array,
    Map,
    Str,
    Protobuf,
    Bytes,
    Enum,
    Opt,
    Primitive,
}
// auto generate, do not edit
pub fn category_from_str(type_str: &str) -> TypeCategory {
    match type_str {
        "Vec" => TypeCategory::Array,
        "HashMap" => TypeCategory::Map,
        "u8" => TypeCategory::Bytes,
        "String" => TypeCategory::Str,
        "KeyValue"
        | "QueryAppRequest"
        | "QueryAppParams"
        | "CreateAppRequest"
        | "ColorStyle"
        | "CreateAppParams"
        | "App"
        | "RepeatedApp"
        | "UpdateAppRequest"
        | "UpdateAppParams"
        | "DeleteAppRequest"
        | "DeleteAppParams"
        | "UpdateWorkspaceRequest"
        | "UpdateWorkspaceParams"
        | "DeleteWorkspaceRequest"
        | "DeleteWorkspaceParams"
        | "CreateWorkspaceRequest"
        | "CreateWorkspaceParams"
        | "Workspace"
        | "RepeatedWorkspace"
        | "QueryWorkspaceRequest"
        | "QueryWorkspaceParams"
        | "CurrentWorkspace"
        | "TrashIdentifier"
        | "CreateTrashParams"
        | "Trash"
        | "RepeatedTrash"
        | "UpdateViewRequest"
        | "UpdateViewParams"
        | "DeleteViewRequest"
        | "DeleteViewParams"
        | "QueryViewRequest"
        | "QueryViewParams"
        | "OpenViewRequest"
        | "CreateViewRequest"
        | "CreateViewParams"
        | "View"
        | "RepeatedView"
        | "WorkspaceError"
        | "WsError"
        | "WsMessage"
        | "CreateDocParams"
        | "Doc"
        | "UpdateDocParams"
        | "DocDelta"
        | "NewDocUser"
        | "QueryDocParams"
        | "RevId"
        | "Revision"
        | "RevisionRange"
        | "WsDocumentData"
        | "DocError"
        | "FFIRequest"
        | "FFIResponse"
        | "ObservableSubject"
        | "SignInRequest"
        | "SignInParams"
        | "SignInResponse"
        | "SignUpRequest"
        | "SignUpParams"
        | "SignUpResponse"
        | "UserToken"
        | "UserProfile"
        | "UpdateUserRequest"
        | "UpdateUserParams"
        | "UserError"
        => TypeCategory::Protobuf,
        "ViewType"
        | "WorkspaceEvent"
        | "ErrorCode"
        | "Notification"
        | "WsModule"
        | "RevType"
        | "WsDataType"
        | "DocObservable"
        | "FFIStatusCode"
        | "UserEvent"
        | "UserObservable"
        => TypeCategory::Enum,

        "Option" => TypeCategory::Opt,
        _ => TypeCategory::Primitive,
    }
}
