// Schema 类型定义
// 用于表示数据库结构信息

use serde::{Deserialize, Serialize};

/// 表的完整结构信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableSchema {
    /// 表名
    pub name: String,
    /// 表所属的 schema (通常是 "public")
    pub schema: String,
    /// 列信息列表
    pub columns: Vec<ColumnInfo>,
    /// 主键列名列表
    pub primary_keys: Vec<String>,
    /// 外键约束列表
    pub foreign_keys: Vec<ForeignKeyInfo>,
    /// 索引列表
    pub indexes: Vec<IndexInfo>,
    /// 表注释
    pub comment: Option<String>,
}

/// 列信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    /// 列名
    pub name: String,
    /// PostgreSQL 数据类型 (例如: "integer", "varchar", "timestamp")
    pub data_type: String,
    /// UDT 类型（用户定义类型）
    pub udt_name: String,
    /// 是否可以为 NULL
    pub is_nullable: bool,
    /// 默认值
    pub default_value: Option<String>,
    /// 是否是自增列 (SERIAL, IDENTITY)
    pub is_identity: bool,
    /// 最大长度（对于 varchar 等类型）
    pub max_length: Option<i32>,
    /// 数值精度（对于 numeric, decimal）
    pub numeric_precision: Option<i32>,
    /// 数值小数位数
    pub numeric_scale: Option<i32>,
    /// 列的位置（1-based）
    pub ordinal_position: i32,
    /// 列注释
    pub comment: Option<String>,
}

/// 外键约束信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForeignKeyInfo {
    /// 约束名称
    pub constraint_name: String,
    /// 本表的列名
    pub column_name: String,
    /// 引用的表名
    pub foreign_table_name: String,
    /// 引用的列名
    pub foreign_column_name: String,
    /// 删除时的行为 (CASCADE, SET NULL, etc.)
    pub on_delete: Option<String>,
    /// 更新时的行为
    pub on_update: Option<String>,
}

/// 索引信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexInfo {
    /// 索引名称
    pub name: String,
    /// 索引包含的列名列表
    pub columns: Vec<String>,
    /// 是否是唯一索引
    pub is_unique: bool,
    /// 是否是主键索引
    pub is_primary: bool,
    /// 索引类型 (btree, hash, gist, etc.)
    pub index_type: String,
}

/// 约束类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConstraintType {
    PrimaryKey,
    ForeignKey,
    Unique,
    Check,
}

/// 数据库 Schema 概览
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaOverview {
    /// Schema 名称 (例如: "public")
    pub name: String,
    /// 该 schema 下的表列表
    pub tables: Vec<String>,
    /// 表数量
    pub table_count: usize,
}

impl TableSchema {
    /// 获取指定列的信息
    pub fn get_column(&self, name: &str) -> Option<&ColumnInfo> {
        self.columns.iter().find(|c| c.name == name)
    }

    /// 检查列是否存在
    pub fn has_column(&self, name: &str) -> bool {
        self.columns.iter().any(|c| c.name == name)
    }

    /// 检查是否是主键列
    pub fn is_primary_key(&self, column_name: &str) -> bool {
        self.primary_keys.iter().any(|pk| pk == column_name)
    }

    /// 获取所有可空列
    pub fn nullable_columns(&self) -> Vec<&ColumnInfo> {
        self.columns.iter().filter(|c| c.is_nullable).collect()
    }

    /// 获取所有必填列（不可空且无默认值）
    pub fn required_columns(&self) -> Vec<&ColumnInfo> {
        self.columns
            .iter()
            .filter(|c| !c.is_nullable && c.default_value.is_none() && !c.is_identity)
            .collect()
    }
}

impl ColumnInfo {
    /// 是否是数值类型
    pub fn is_numeric(&self) -> bool {
        matches!(
            self.data_type.as_str(),
            "integer" | "bigint" | "smallint" | "numeric" | "decimal" | "real" | "double precision"
        )
    }

    /// 是否是字符串类型
    pub fn is_text(&self) -> bool {
        matches!(
            self.data_type.as_str(),
            "character varying" | "varchar" | "character" | "char" | "text"
        )
    }

    /// 是否是时间类型
    pub fn is_temporal(&self) -> bool {
        matches!(
            self.data_type.as_str(),
            "timestamp"
                | "timestamptz"
                | "timestamp with time zone"
                | "timestamp without time zone"
                | "date"
                | "time"
        )
    }

    /// 是否是布尔类型
    pub fn is_boolean(&self) -> bool {
        self.data_type == "boolean"
    }

    /// 是否是 JSON 类型
    pub fn is_json(&self) -> bool {
        matches!(self.data_type.as_str(), "json" | "jsonb")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_schema_methods() {
        let schema = TableSchema {
            name: "users".to_string(),
            schema: "public".to_string(),
            columns: vec![
                ColumnInfo {
                    name: "id".to_string(),
                    data_type: "integer".to_string(),
                    udt_name: "int4".to_string(),
                    is_nullable: false,
                    default_value: None,
                    is_identity: true,
                    max_length: None,
                    numeric_precision: Some(32),
                    numeric_scale: Some(0),
                    ordinal_position: 1,
                    comment: None,
                },
                ColumnInfo {
                    name: "email".to_string(),
                    data_type: "varchar".to_string(),
                    udt_name: "varchar".to_string(),
                    is_nullable: false,
                    default_value: None,
                    is_identity: false,
                    max_length: Some(255),
                    numeric_precision: None,
                    numeric_scale: None,
                    ordinal_position: 2,
                    comment: None,
                },
            ],
            primary_keys: vec!["id".to_string()],
            foreign_keys: vec![],
            indexes: vec![],
            comment: None,
        };

        assert!(schema.has_column("id"));
        assert!(schema.has_column("email"));
        assert!(!schema.has_column("password"));

        assert!(schema.is_primary_key("id"));
        assert!(!schema.is_primary_key("email"));

        let required = schema.required_columns();
        assert_eq!(required.len(), 1);
        assert_eq!(required[0].name, "email");
    }

    #[test]
    fn test_column_type_checks() {
        let int_col = ColumnInfo {
            name: "count".to_string(),
            data_type: "integer".to_string(),
            udt_name: "int4".to_string(),
            is_nullable: false,
            default_value: None,
            is_identity: false,
            max_length: None,
            numeric_precision: Some(32),
            numeric_scale: Some(0),
            ordinal_position: 1,
            comment: None,
        };

        assert!(int_col.is_numeric());
        assert!(!int_col.is_text());
        assert!(!int_col.is_temporal());

        let text_col = ColumnInfo {
            name: "description".to_string(),
            data_type: "text".to_string(),
            udt_name: "text".to_string(),
            is_nullable: true,
            default_value: None,
            is_identity: false,
            max_length: None,
            numeric_precision: None,
            numeric_scale: None,
            ordinal_position: 2,
            comment: None,
        };

        assert!(!text_col.is_numeric());
        assert!(text_col.is_text());
        assert!(!text_col.is_temporal());
    }
}
