import { CollectionPaginationResponse, EntityOptionInterface } from "../types/EntityType";
import http from "../utils/http";

class EntityApi {
  async fetchEntitiesOptions() {
    try {
      const response = await http.get<Array<EntityOptionInterface>>("/api/entities/options");
      return response;
    } catch (error) {
      console.error(error);
    }
  }

  async fetchCollectionsByEntityId(entityId: string, page: number = 1, pageSize: number = 20) {
    try {
      const response = await http.get<CollectionPaginationResponse>(`/api/collection/${entityId}`, {
        params: {
          page,
          page_size: pageSize,
        },
      });
      return response;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }
}

export const entityApi = new EntityApi();
