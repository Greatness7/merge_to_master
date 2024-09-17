use merge_to_master::prelude::*;

use anyhow::Result;
use std::collections::HashMap;

const OPTIONS: MergeOptions = MergeOptions { remove_deleted: false };

/// Mapping of { dialogue_id => { info_id => [prev_id, next_id] } }
///
type DialogueData = HashMap<String, HashMap<String, [String; 2]>>;

fn load_dialogue_data(path: impl AsRef<Path>) -> Result<DialogueData> {
    let mut file = std::fs::File::open(path)?;
    let mut data = rkyv::AlignedVec::new();
    data.extend_from_reader(&mut file)?;
    Ok(unsafe { rkyv::from_bytes_unchecked(&data)? })
}

#[test]
fn merge_references_1() {
    let plugin_path = PathBuf::from("./tests/assets/merge_references_1/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/merge_references_1/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/merge_references_1/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_insert_empty() {
    let plugin_path = PathBuf::from("./tests/assets/info_insert_empty/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_insert_empty/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_insert_empty/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_insert_front() {
    let plugin_path = PathBuf::from("./tests/assets/info_insert_front/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_insert_front/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_insert_front/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_insert_middle() {
    let plugin_path = PathBuf::from("./tests/assets/info_insert_middle/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_insert_middle/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_insert_middle/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_insert_end() {
    let plugin_path = PathBuf::from("./tests/assets/info_insert_end/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_insert_end/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_insert_end/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_insert_replacing() {
    let plugin_path = PathBuf::from("./tests/assets/info_insert_replacing/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_insert_replacing/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_insert_replacing/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_preserve_gaps() {
    let plugin_path = PathBuf::from("./tests/assets/info_preserve_gaps/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_preserve_gaps/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_preserve_gaps/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn test_merged_dialogue_mw() -> Result<()> {
    let master_path = PathBuf::from("./ignore/MW/Master.esm");
    let plugin_path = PathBuf::from("./ignore/MW/Plugin.esp");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;

    let dialogues_merged = merged.dialogues;
    let dialogues_expect = load_dialogue_data("./ignore/MW/Expect.rkyv")?;
    assert_eq!(dialogues_merged.len(), dialogues_expect.len());

    for (id, dialogue_group) in dialogues_merged {
        let infos_merged = &dialogue_group.infos;
        let infos_expect = &dialogues_expect[&id];
        assert_eq!(infos_merged.len(), infos_expect.len());

        for info in &dialogue_group.infos {
            let [prev_id, next_id] = &infos_expect[&info.id];
            assert_eq!(&info.prev_id, prev_id);
            assert_eq!(&info.next_id, next_id);
        }
    }

    Ok(())
}

#[test]
fn test_merged_dialogue_mw_tb() -> Result<()> {
    let master_path = PathBuf::from("./ignore/MW_TB/Master.esm");
    let plugin_path = PathBuf::from("./ignore/MW_TB/Plugin.esp");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;

    let dialogues_merged = merged.dialogues;
    let dialogues_expect = load_dialogue_data("./ignore/MW_TB/Expect.rkyv")?;
    assert_eq!(dialogues_merged.len(), dialogues_expect.len());

    for (id, dialogue_group) in dialogues_merged {
        let infos_merged = &dialogue_group.infos;
        let infos_expect = &dialogues_expect[&id];
        assert_eq!(infos_merged.len(), infos_expect.len());

        for info in &dialogue_group.infos {
            let [prev_id, next_id] = &infos_expect[&info.id];
            assert_eq!(&info.prev_id, prev_id);
            assert_eq!(&info.next_id, next_id);
        }
    }

    Ok(())
}

#[test]
fn test_merged_dialogue_mw_bm() -> Result<()> {
    let master_path = PathBuf::from("./ignore/MW_BM/Master.esm");
    let plugin_path = PathBuf::from("./ignore/MW_BM/Plugin.esp");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;

    let dialogues_merged = merged.dialogues;
    let dialogues_expect = load_dialogue_data("./ignore/MW_BM/Expect.rkyv")?;
    assert_eq!(dialogues_merged.len(), dialogues_expect.len());

    for (id, dialogue_group) in dialogues_merged {
        let infos_merged = &dialogue_group.infos;
        let infos_expect = &dialogues_expect[&id];
        assert_eq!(infos_merged.len(), infos_expect.len());

        for info in &dialogue_group.infos {
            let [prev_id, next_id] = &infos_expect[&info.id];
            assert_eq!(&info.prev_id, prev_id);
            assert_eq!(&info.next_id, next_id);
        }
    }

    Ok(())
}

#[test]
fn test_merged_dialogue_mw_tb_bm() -> Result<()> {
    let master_path = PathBuf::from("./ignore/MW_TB_BM/Master.esm");
    let plugin_path = PathBuf::from("./ignore/MW_TB_BM/Plugin.esp");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;

    let dialogues_merged = merged.dialogues;
    let dialogues_expect = load_dialogue_data("./ignore/MW_TB_BM/Expect.rkyv")?;
    assert_eq!(dialogues_merged.len(), dialogues_expect.len());

    for (id, dialogue_group) in dialogues_merged {
        let infos_merged = &dialogue_group.infos;
        let infos_expect = &dialogues_expect[&id];
        assert_eq!(infos_merged.len(), infos_expect.len());

        for info in infos_merged {
            let [prev_id, next_id] = &infos_expect[&info.id];
            assert_eq!(&info.prev_id, prev_id);
            assert_eq!(&info.next_id, next_id);
        }
    }

    Ok(())
}

#[test]
fn test_merged_dialogue_tb_bm() -> Result<()> {
    let plugin_path = PathBuf::from("./ignore/TB_BM/Plugin.esp");
    let master_path = PathBuf::from("./ignore/TB_BM/Master.esm");

    let merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;

    let dialogues_merged = merged.dialogues;
    let dialogues_expect = load_dialogue_data("./ignore/TB_BM/Expect.rkyv")?;
    assert_eq!(dialogues_merged.len(), dialogues_expect.len());

    for (id, dialogue_group) in dialogues_merged {
        let infos_merged = &dialogue_group.infos;
        let infos_expect = &dialogues_expect[&id];
        assert_eq!(infos_merged.len(), infos_expect.len());

        for info in infos_merged {
            let [prev_id, next_id] = &infos_expect[&info.id];
            assert_eq!(&info.prev_id, prev_id);
            assert_eq!(&info.next_id, next_id);
        }
    }

    Ok(())
}

#[test]
fn remove_deleted() {
    let plugin_path = PathBuf::from("./tests/assets/remove_deleted/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/remove_deleted/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/remove_deleted/Expect.esm");

    let options = MergeOptions { remove_deleted: true };

    let merged = merge_plugins(&plugin_path, &master_path, options).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}
