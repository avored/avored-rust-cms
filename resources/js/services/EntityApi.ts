import { CollectionPaginationResponse, EntityInterface, EntityOptionInterface } from "../types/EntityType";
import http from "../utils/http";

class EntityApi {
  async fetchEntitiesOptions() {
    try {
      const response = await http.get<Array<EntityOptionInterface>>("/api/v1/admin/entities/options");
      return response;
    } catch (error) {
      console.error(error);
    }
  }

  async fetchEntityById(entityId: string) {
    try {
      const response = await http.get<EntityInterface>(`/api/v1/admin/entities/${entityId}`);
      return response;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  async fetchCollectionsByEntityId(entityId: string, page: number = 1, pageSize: number = 25) {
    try {
      const response = await http.get<CollectionPaginationResponse>("/api/v1/admin/collection", {
        params: {
          entity_id: entityId,
          page,
          per_page: pageSize,
        },
      });
      return response;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  async fetchCollectionById(entityId: string, recordId: string) {
    try {
      const response = await http.get<Record<string, any>>(`/api/v1/admin/collection/${recordId}?entity_id=${encodeURIComponent(entityId)}`);
      return response;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  async createCollection(entityId: string, data: Record<string, any>) {
    try {
      const response = await http.post<Record<string, any>>(`/api/v1/admin/collection?entity_id=${encodeURIComponent(entityId)}`, data);
      return response;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  async updateCollection(entityId: string, recordId: string, data: Record<string, any>) {
    try {
      const response = await http.put<Record<string, any>>(`/api/v1/admin/collection/${recordId}?entity_id=${encodeURIComponent(entityId)}`, data);
      return response;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }

  async deleteCollection(entityId: string, recordId: string) {
    try {
      const response = await http.delete<{ success: boolean }>(`/api/v1/admin/collection/${recordId}?entity_id=${entityId}`);
      return response;
    } catch (error) {
      console.error(error);
      throw error;
    }
  }
}

export const entityApi = new EntityApi();
