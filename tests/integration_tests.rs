use merge_to_master::prelude::*;

const OPTIONS: MergeOptions = MergeOptions {
    remove_deleted: false,
    apply_moved_references: false,
    preserve_duplicate_references: false,
};

const REMOVE_DELETED: MergeOptions = MergeOptions {
    remove_deleted: true,
    ..OPTIONS
};

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
fn remove_deleted() {
    let plugin_path = PathBuf::from("./tests/assets/remove_deleted/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/remove_deleted/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/remove_deleted/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, REMOVE_DELETED).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn remove_deleted_fields() {
    let plugin_path = PathBuf::from("./tests/assets/remove_deleted_fields/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/remove_deleted_fields/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/remove_deleted_fields/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, REMOVE_DELETED).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn remove_deleted_references() {
    let plugin_path = PathBuf::from("./tests/assets/remove_deleted_references/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/remove_deleted_references/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/remove_deleted_references/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, REMOVE_DELETED).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn rename_cells() {
    let plugin_path = PathBuf::from("./tests/assets/rename_cells/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/rename_cells/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/rename_cells/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, REMOVE_DELETED).unwrap();
    let expect = PluginData::from_path(&expect_path).unwrap();

    for (key, merged_object) in &merged.objects {
        let expect_object = expect.objects.get(key).unwrap();

        use tes3::esp::*;

        let merged_references = match merged_object {
            TES3Object::Cell(cell) => cell.references.values().sorted_by_key(|r| &r.id),
            _ => unreachable!(),
        };

        let expect_references = match expect_object {
            TES3Object::Cell(cell) => cell.references.values().sorted_by_key(|r| &r.id),
            _ => unreachable!(),
        };

        for (merged_ref, expect_ref) in merged_references.zip(expect_references) {
            assert_eq!(merged_ref.id, expect_ref.id);
            assert_eq!(merged_ref.translation, expect_ref.translation);
            assert_eq!(merged_ref.rotation, expect_ref.rotation);
            assert_eq!(merged_ref.scale, expect_ref.scale);
        }
    }
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
fn info_delete_front() {
    let plugin_path = PathBuf::from("./tests/assets/info_delete_front/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_delete_front/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_delete_front/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, REMOVE_DELETED).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_delete_end() {
    let plugin_path = PathBuf::from("./tests/assets/info_delete_end/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_delete_end/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_delete_end/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, REMOVE_DELETED).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_delete_middle() {
    let plugin_path = PathBuf::from("./tests/assets/info_delete_middle/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_delete_middle/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_delete_middle/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, REMOVE_DELETED).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

#[test]
fn info_delete_middle_2() {
    let plugin_path = PathBuf::from("./tests/assets/info_delete_middle_2/Plugin.esp");
    let master_path = PathBuf::from("./tests/assets/info_delete_middle_2/Master.esm");
    let expect_path = PathBuf::from("./tests/assets/info_delete_middle_2/Expect.esm");

    let merged = merge_plugins(&plugin_path, &master_path, REMOVE_DELETED).unwrap();

    let merged_bytes = merged.into_plugin().save_bytes().unwrap();
    let expect_bytes = std::fs::read(expect_path).unwrap();

    assert_eq!(merged_bytes, expect_bytes);
}

// ---------------------------------------------------------------------------

use std::collections::HashMap as _HashMap;

/// Mapping of { dialogue_id => { info_id => [prev_id, next_id] } }
///
type DialogueData = _HashMap<String, _HashMap<String, [String; 2]>>;

fn load_dialogue_data(path: impl AsRef<Path>) -> Result<DialogueData> {
    let mut file = std::fs::File::open(path)?;
    let mut data = rkyv::AlignedVec::new();
    data.extend_from_reader(&mut file)?;
    Ok(unsafe { rkyv::from_bytes_unchecked(&data)? })
}

#[test]
fn test_merged_dialogue_mw() -> Result<()> {
    let master_path = PathBuf::from("./ignore/MW/Master.esm");
    let plugin_path = PathBuf::from("./ignore/MW/Plugin.esp");

    let mut merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;
    merged.remove_ignored();

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

    let mut merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;
    merged.remove_ignored();

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

    let mut merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;
    merged.remove_ignored();

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

    let mut merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;
    merged.remove_ignored();

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

    let mut merged = merge_plugins(&plugin_path, &master_path, OPTIONS)?;
    merged.remove_ignored();

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
