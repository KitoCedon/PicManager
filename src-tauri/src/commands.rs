use crate::config::{Bed, Config};
#[tauri::command]
pub fn upload(){
    // 获取文件位置
    
    // API初始化

    // 上传
}

#[tauri::command]
pub fn update_bed(){
    /*
     * 更新图床缓存
     */
    
    // API初始化

    // 获取文件列表

    // 生成,写入缓存
}

#[tauri::command]
pub fn get_bed_list(app: tauri::AppHandle) -> Vec<Bed> {
    /*
     * 获取图床列表
     * 返回数组表
     */
    Config::load(&app).beds
}
