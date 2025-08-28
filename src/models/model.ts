export type LiteratureNoteModel = {
    id: string;
    content: string;
    create_time: Date;
    update_time: Date;
};

export type TagModel = {
    id: string;
    full_name: string;
    p_id: string;
    create_time: Date;
    update_time: Date;
    is_new: boolean;
}
