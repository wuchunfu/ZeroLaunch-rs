<template>
    <div class="storage-config-container" style="height: 100%; overflow-y: auto;">
        <h2 class="page-title">配置文件存储设置</h2>

        <el-form :model="formData" label-width="180px" class="storage-config-form">
            <el-form-item label="配置文件存储位置" class="storage-type-selector">
                <el-radio-group v-model="formData.storage_destination" size="large">
                    <el-radio-button value="Local">本地存储</el-radio-button>
                    <el-radio-button value="WebDAV">WebDAV</el-radio-button>
                    <!--<el-radio-button value="OneDrive">OneDrive</el-radio-button> -->
                </el-radio-group>
            </el-form-item>

            <!-- 存储配置区块 -->
            <div class="config-section">
                <div v-if="formData.storage_destination === 'Local'" class="storage-section">
                    <h3 class="section-title">本地存储设置</h3>
                    <el-form-item label="配置文件保存路径">
                        <div class="path-input-container">
                            <el-input v-model="formData.local_save_config.destination_dir" placeholder="配置文件保存路径"
                                readonly class="path-display" :title="formData.local_save_config.destination_dir">
                                <template #prefix>
                                    <el-icon>
                                        <Folder />
                                    </el-icon>
                                </template>
                            </el-input>
                            <div class="path-buttons">
                                <el-button type="primary" @click="handleChangeConfigPath">
                                    <el-icon>
                                        <FolderOpened />
                                    </el-icon> 选择路径
                                </el-button>
                                <el-button @click="handleUseDefaultPath">
                                    <el-icon>
                                        <SetUp />
                                    </el-icon> 使用默认路径
                                </el-button>
                            </div>
                        </div>
                    </el-form-item>
                </div>
            </div>

            <!-- WebDAV 配置 -->
            <div v-if="formData.storage_destination === 'WebDAV'" class="storage-section">
                <h3 class="section-title">WebDAV 设置</h3>
                <el-form-item label="服务器地址">
                    <el-input v-model="formData.webdav_save_config.host_url" placeholder="请输入 WebDAV 服务器地址">
                        <template #prefix>
                            <el-icon>
                                <Link />
                            </el-icon>
                        </template>
                    </el-input>
                </el-form-item>
                <el-form-item label="账号">
                    <el-input v-model="formData.webdav_save_config.account" placeholder="请输入账号">
                        <template #prefix>
                            <el-icon>
                                <User />
                            </el-icon>
                        </template>
                    </el-input>
                </el-form-item>
                <el-form-item label="密码">
                    <el-input v-model="formData.webdav_save_config.password" type="password" placeholder="请输入密码"
                        show-password>
                        <template #prefix>
                            <el-icon>
                                <Lock />
                            </el-icon>
                        </template>
                    </el-input>
                </el-form-item>
                <el-form-item label="目标目录">
                    <el-input v-model="formData.webdav_save_config.destination_dir" placeholder="请输入目标目录">
                        <template #prefix>
                            <el-icon>
                                <FolderOpened />
                            </el-icon>
                        </template>
                    </el-input>
                </el-form-item>
            </div>

            <!-- OneDrive 配置 -->
            <!--
                <div v-if="formData.storage_destination === 'OneDrive'" class="storage-section">
                    <h3 class="section-title">
                        OneDrive 设置
                    </h3>
                    <el-alert v-if="formData.onedrive_save_config.refresh_token" type="success" show-icon
                        :closable="false" class="auth-status-alert">
                        <span>OneDrive 已授权</span>
                    </el-alert>
                    <el-alert v-else type="info" show-icon :closable="false" class="auth-status-alert">
                        <span>请完成 OneDrive 授权</span>
                    </el-alert>
                    <el-form-item label="文件夹路径">
                        <el-input v-model="formData.onedrive_save_config.destination_dir"
                            placeholder="请输入 OneDrive 文件夹路径">
                            <template #prefix>
                                <el-icon>
                                    <FolderOpened />
                                </el-icon>
                            </template>
                        </el-input>
                    </el-form-item>

                    <el-form-item>
                        <el-button type="primary" @click="authorizeOneDrive" :disabled="!!auth_link"
                            class="auth-button">
                            <el-icon>
                                <Key />
                            </el-icon>
                            {{ formData.onedrive_save_config.refresh_token ? '重新授权' : '授权 OneDrive' }}
                        </el-button>
                    </el-form-item>

                    <el-form-item v-if="auth_link" class="auth-link-container">
                        <el-card shadow="hover" class="auth-card">
                            <template #header>
                                <div class="auth-card-header">
                                    <el-icon>
                                        <Link />
                                    </el-icon>
                                    <span>请在浏览器中完成授权</span>
                                </div>
                            </template>
                            <div class="auth-link-content">
                                <p>点击下方按钮在浏览器中打开授权页面：</p>
                                <el-button type="primary" @click="openAuthLink" class="open-link-button">
                                    <el-icon>
                                        <Link />
                                    </el-icon> 打开授权页面
                                </el-button>
                                <div class="auth-link-text">
                                    <p>或复制以下链接到浏览器打开：</p>
                                    <el-input v-model="auth_link" readonly class="auth-link-input">
                                        <template #append>
                                            <el-button @click="copyAuthLink">
                                                <el-icon>
                                                    <Document />
                                                </el-icon> 复制
                                            </el-button>
                                        </template>
                                    </el-input>
                                </div>
                                <p class="auth-note">完成授权后请返回应用继续操作</p>
                            </div>
                        </el-card>
                    </el-form-item>
                </div>
            </div>
            -->

            <!-- 通用设置区块 -->
            <div class="config-section">
                <h3 class="section-title">通用设置</h3>
                <el-form-item label="配置缓存次数">
                    <el-input-number v-model="formData.save_to_local_per_update" :min="0" :step="1" :precision="0"
                        controls-position="right" class="number-input" />
                    <span class="input-description">设置为0表示每次保存配置都上传</span>
                </el-form-item>
            </div>

            <div class="action-buttons">
                <el-button type="primary" @click="testConfigValidation">
                    <el-icon>
                        <Connection />
                    </el-icon> 测试连接
                </el-button>
                <el-button type="primary" @click="saveConfig" :disabled="!allowSave">
                    <el-icon>
                        <Collection />
                    </el-icon> 保存配置
                </el-button>
                <el-button @click="resetConfig">
                    <el-icon>
                        <RefreshRight />
                    </el-icon> 重置
                </el-button>
            </div>
        </el-form>
    </div>
</template>

<script lang="ts" setup>
import { ref, reactive, onMounted, watch } from 'vue'
import { useLocalConfigStore } from '../stores/local_config'
import { ElMessage } from 'element-plus'
import {
    Folder, FolderOpened, SetUp, Link, User, Lock,
    Connection, Collection, RefreshRight
} from '@element-plus/icons-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { PartialLocalConfig } from '../api/local_config_types'
import { listen, UnlistenFn } from '@tauri-apps/api/event'
const auth_link = ref('')
const allowSave = ref(false)
// 获取配置存储
const configStore = useLocalConfigStore()

// 表单数据
const formData = reactive({
    storage_destination: configStore.config.storage_destination,
    local_save_config: { ...configStore.config.local_save_config },
    webdav_save_config: { ...configStore.config.webdav_save_config },
    // onedrive_save_config: { ...configStore.config.onedrive_save_config },
    save_to_local_per_update: configStore.config.save_to_local_per_update
})

watch(
    () => formData,
    () => {
        allowSave.value = false
    },
    { deep: true }
)
let unlisten: Array<UnlistenFn | null> = [];
// 初始化
onMounted(async () => {
    await configStore.loadConfig();
    Object.assign(formData, configStore.config);
    unlisten.push(await listen('emit_update_auth_link', async (event) => {
        auth_link.value = event.payload as string;
    }))
})

// 选择本地配置文件路径
const handleChangeConfigPath = async () => {
    try {
        const selected = await open({
            canCreateDirectories: true,
            directory: true,
            multiple: false,
            title: "选择配置文件保存位置"
        });

        if (selected) {
            formData.local_save_config.destination_dir = selected;
        }
    } catch (error) {
        handleError('选择文件夹失败', error);
    }

    console.log('打开文件选择对话框')
}

// 使用默认路径
const handleUseDefaultPath = async () => {
    const default_path = await invoke<string>('command_get_default_remote_data_dir_path');
    formData.local_save_config.destination_dir = default_path
    ElMessage.success('已设置为默认路径')
}

const testConfigValidation = async () => {
    try {
        const validation = await invoke<PartialLocalConfig>('command_check_validation', { partialConfig: formData });
        console.log(validation);
        if (validation) {
            ElMessage.success('连接成功')
            allowSave.value = true // 测试成功后允许保存
            configStore.updateConfig(validation)
        } else {
            ElMessage.error('连接失败')
            allowSave.value = false
        }

    } catch (error) {
        ElMessage.error('连接失败')
        allowSave.value = false
    }
}

// // 授权 OneDrive
// const authorizeOneDrive = async () => {
//     try {
//         const refresh_token = await invoke<string>('command_get_onedrive_refresh_token')
//         formData.onedrive_save_config.refresh_token = refresh_token;
//         ElMessage.success('OneDrive 授权成功')
//         auth_link.value = ''
//     } catch (error) {
//         handleError('获取授权链接失败', error)
//     }
// }

// // 在浏览器中打开授权链接
// const openAuthLink = async () => {
//     if (auth_link.value) {
//         try {
//             await openUrl(auth_link.value)
//         } catch (error) {
//             handleError('打开浏览器失败', error)
//         }
//     }
// }

// // 复制授权链接
// const copyAuthLink = () => {
//     if (auth_link.value) {
//         navigator.clipboard.writeText(auth_link.value)
//             .then(() => ElMessage.success('链接已复制到剪贴板'))
//             .catch(() => ElMessage.error('复制失败'))
//     }
// }


// 保存配置
const saveConfig = async () => {
    try {
        await configStore.updateConfig(formData)
        configStore.syncConfig();
        ElMessage.success('配置已保存')
    } catch (error) {
        handleError('保存配置失败', error)
    }
}

// 重置配置
const resetConfig = () => {
    Object.assign(formData, {
        storage_destination: configStore.config.storage_destination,
        local_save_config: { ...configStore.config.local_save_config },
        webdav_save_config: { ...configStore.config.webdav_save_config },
        // onedrive_save_config: { ...configStore.config.onedrive_save_config },
        save_to_local_per_update: configStore.config.save_to_local_per_update
    })
    auth_link.value = ''
    ElMessage.info('配置已重置')
}

const handleError = (message: string, error: unknown) => {
    console.error(message, error)
    ElMessage.error(message)
};

</script>

<style scoped>
.storage-config-container {
    padding: 20px;
    max-width: 800px;
    margin: 0 auto;
}

.page-title {
    margin-bottom: 24px;
    color: var(--el-text-color-primary);
    font-weight: 600;
    border-bottom: 1px solid var(--el-border-color-light);
    padding-bottom: 12px;
}

.storage-section {
    background-color: var(--el-fill-color-light);
    border-radius: 8px;
    padding: 20px;
    margin-bottom: 24px;
    box-shadow: 0 2px 12px 0 rgba(0, 0, 0, 0.05);
}

.section-title {
    display: flex;
    align-items: center;
    gap: 8px;
    margin-bottom: 20px;
    color: var(--el-text-color-primary);
    font-weight: 500;
}

.input-description {
    margin-left: 12px;
    color: var(--el-text-color-secondary);
    font-size: 14px;
}

.action-buttons {
    display: flex;
    gap: 12px;
    margin-top: 24px;
    justify-content: flex-start;
}


.config-section {
    margin-top: 24px;
    padding-top: 24px;
    border-top: 1px solid var(--el-border-color-light);
}

.storage-config-form {
    max-width: 100%;
}

.storage-type-selector {
    margin-bottom: 25px;
    display: flex;
    justify-content: center;
}

/* 保持其他原有样式不变 */
.path-input-container {
    display: flex;
    flex-direction: column;
    gap: 10px;
    width: 100%;
}

.path-display {
    font-family: 'Courier New', monospace;
    font-size: 14px;
    width: 100%;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}

.path-buttons {
    display: flex;
    gap: 10px;
}

.number-input {
    width: 180px;
}

.input-description {
    margin-left: 15px;
    color: #909399;
    font-size: 14px;
}

.action-buttons {
    display: flex;
    justify-content: center;
    gap: 20px;
    margin-top: 30px;
    margin-bottom: 30px;
}

.path-display :deep(.el-input__wrapper) {
    padding-right: 15px;
}

.path-display :deep(.el-input__inner) {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
}
</style>