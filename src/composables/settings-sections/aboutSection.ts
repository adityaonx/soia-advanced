import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";

export type RuntimeVersions = {
    soiaVersion: string;
    mpvVersion?: string | null;
    ffmpegVersion?: string | null;
};

export const useAboutSection = () => {
    const runtimeVersions = ref<RuntimeVersions | null>(null);

    const loadRuntimeVersions = async () => {
        try {
            runtimeVersions.value = await invoke<RuntimeVersions>(
                "get_runtime_versions",
            );
        } catch {
            runtimeVersions.value = null;
        }
    };

    return {
        runtimeVersions,
        loadRuntimeVersions,
    };
};
