import 'package:flowy_sdk/protobuf/flowy-workspace/protobuf.dart';
import 'package:dartz/dartz.dart';

typedef AppUpdatedCallback = void Function(String name, String desc);
typedef AppViewsChangeCallback = void Function(Either<List<View>, WorkspaceError> viewsOrFailed);

abstract class IApp {
  Future<Either<List<View>, WorkspaceError>> getViews();

  Future<Either<View, WorkspaceError>> createView({required String name, String? desc, required ViewType viewType});
}

abstract class IAppListenr {
  void start({AppViewsChangeCallback? viewsChangeCallback, AppUpdatedCallback? updatedCallback});

  Future<void> stop();
}
