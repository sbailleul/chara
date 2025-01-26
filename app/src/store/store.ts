import { definitionApi } from "@/routes/definition/-feature/store/api";
import { configureStore } from "@reduxjs/toolkit";

export const store = configureStore({
	reducer: {
		[definitionApi.reducerPath]: definitionApi.reducer,
	},
	middleware: (getDefaultMiddleware) =>
		getDefaultMiddleware().concat(definitionApi.middleware),
});

// Infer the `RootState` and `AppDispatch` types from the store itself
export type RootState = ReturnType<typeof store.getState>;
// Inferred type: {posts: PostsState, comments: CommentsState, users: UsersState}
export type AppDispatch = typeof store.dispatch;
