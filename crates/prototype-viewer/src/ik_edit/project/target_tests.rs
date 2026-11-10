use super::*;
use crate::{
    demo::{DEMO_AGENT, action_id, hand_ik},
    ik_edit::profiles::tests::fixture,
};

#[test]
fn project_linksets_exclude_seated_avatars_and_their_attachments() {
    let (mut world, _, _) = fixture();
    let mut objects = hand_ik::objects(0.0, true);
    let mut attachment = objects[1].clone();
    attachment.local_id = hand_ik::RIGHT + 1;
    attachment.full_id = action_id(attachment.local_id);
    attachment.parent_id = objects.last().unwrap().local_id;
    let attachment_id = attachment.full_id;
    objects.push(attachment);
    world.apply(prototype_net::NetEvent::ObjectUpdates { handle: 1, objects });

    let index = |id| world.objects.index_of_uuid(&id).expect("fixture object");
    let source = action_id(hand_ik::ROOT);
    let left = action_id(hand_ik::LEFT);
    let right = action_id(hand_ik::RIGHT);
    assert_eq!(root(&world, index(left)), Some(index(source)));
    assert_eq!(root(&world, index(DEMO_AGENT)), None);
    assert_eq!(root(&world, index(attachment_id)), Some(index(attachment_id)));
    assert_eq!(
        members(&world, source).into_iter().collect::<HashSet<_>>(),
        HashSet::from([source, left, right])
    );
}

#[test]
fn project_anchor_waits_for_complete_names_and_rejects_ambiguous_matches() {
    let (world, _, mut editor) = fixture();
    let source = action_id(hand_ik::ROOT);
    let left = action_id(hand_ik::LEFT);
    let right = action_id(hand_ik::RIGHT);
    let mut build = BuildTool::default();
    build.props = hand_ik::properties();
    editor.graph.source = Some(source);
    editor.graph.definition.nodes[0].parent = Target::Prim {
        id: left,
        name: String::new(),
    };

    let right_props = build.props.remove(&right).expect("right grip");
    editor.sync_project_target(&world, &build);
    assert_eq!(
        editor.graph.definition.nodes[0].parent,
        Target::Prim {
            id: left,
            name: String::new()
        }
    );

    build.props.insert(right, right_props);
    editor.sync_project_target(&world, &build);
    assert_eq!(
        editor.graph.definition.nodes[0].parent,
        Target::Prim {
            id: left,
            name: "Left".into()
        }
    );

    build.props.get_mut(&right).expect("right grip").name = "Left".into();
    editor.sync_project_target(&world, &build);
    assert_eq!(
        editor.graph.definition.nodes[0].parent,
        Target::Prim {
            id: Uuid::nil(),
            name: "Left".into()
        }
    );
    assert!(editor.project.target.unresolved > 0);
}
