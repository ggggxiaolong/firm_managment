import { inject, type Ref } from "vue";
export const PROJECT_INJECT = "project";

export interface ProjectInject {
  readonly reload: Ref<number>;
  handleError(e: any): void;
}

export function projectInjetct(): ProjectInject {
  return inject<ProjectInject>(PROJECT_INJECT)!!;
}
